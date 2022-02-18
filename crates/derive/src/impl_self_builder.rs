use std::{ops::Not, str::FromStr};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Field, GenericArgument, Ident, Path, PathArguments, Type};

#[derive(Default)]
pub(crate) struct Config {
    pub(crate) for_self: bool,
    pub(crate) strip_option: bool,
    pub(crate) impl_new: bool,
}

pub(crate) fn self_builder(input: &DeriveInput, config: Config) -> TokenStream {
    let name = &input.ident;
    let trait_name = format_ident!("{}SelfBuilder", name);
    let new_impl = config
        .impl_new
        .then(|| construct_new_impl(&config, name, &input.data))
        .unwrap_or_else(TokenStream2::new);
    let fields = match &input.data {
        Data::Struct(s) => s.fields.iter().map(FieldInfo::from).map(|mut f| {
            if config.strip_option {
                f.strip_option = true;
            }
            f
        }),
        _ => return TokenStream::new(),
    };
    let impls_signs = fields
        .flat_map(|f| construct_impl_sign(f, config.for_self))
        .collect::<Vec<_>>();

    let mut impls = Vec::with_capacity(impls_signs.len());
    let mut signs = Vec::with_capacity(impls_signs.len());
    for (impll, sign) in impls_signs {
        impls.push(impll);
        signs.push(sign);
    }

    if config.for_self {
        (quote! {
            impl #name {
                #new_impl

                #(
                    #impls
                )*
            }
        })
        .into()
    } else {
        let doc_msg = format!("Builder trait for {}.", name);
        (quote! {
            #[doc = #doc_msg]
            pub trait #trait_name {
                #(
                    #signs
                )*
            }

            impl #trait_name for #name {
                #(
                    #impls
                )*
            }
        })
        .into()
    }
}

// TODO: implement method rename `builder(setter(name = "new_something"))`
struct FieldInfo {
    name: Ident,
    ty: Type,
    is_optional: bool,
    strip_option: bool,
    skip_setter: bool,
}

impl From<&Field> for FieldInfo {
    fn from(field: &Field) -> Self {
        let mut strip_option = false;
        let mut skip_setter = false;
        let mut is_optional = false;
        for attr in field.attrs.iter() {
            let attr = attr.tokens.to_string();
            match attr.as_str() {
                "builder(setter(strip_option))" => strip_option = true,
                "builder(setter(skip))" => skip_setter = true,
                _ => {}
            }
            if attr.contains("optional") {
                is_optional = true;
            }
        }

        Self {
            name: field.ident.as_ref().expect("Expected a name").clone(),
            ty: field.ty.clone(),
            is_optional,
            strip_option,
            skip_setter,
        }
    }
}

fn construct_new_impl(config: &Config, name: &Ident, data: &Data) -> TokenStream2 {
    match data {
        Data::Struct(s) => {
            let fields = s.fields.iter().map(FieldInfo::from).map(|mut f| {
                if config.strip_option {
                    f.strip_option = true;
                }
                f
            });

            let mut args = TokenStream2::new();
            let mut names = TokenStream2::new();

            for field_info in fields {
                let FieldInfo {
                    name,
                    ty,
                    is_optional,
                    strip_option,
                    ..
                } = field_info;

                if is_optional.not() {
                    let opt_ty = extract_type_from_option(&ty);
                    let ty = strip_option.then(|| opt_ty.clone()).flatten().unwrap_or(ty);
                    let arg = quote! { #name : impl Into<#ty>, };
                    let value = (opt_ty.is_some() && strip_option)
                        .then(|| quote! { Some(#name .into()) })
                        .unwrap_or_else(|| quote! { #name .into() });
                    let name = quote! { #name : #value, };
                    args.extend(arg);
                    names.extend(name);
                }
            }

            let doc_msg = format!("Create a new [`{}`].", name);

            quote! {
                #[doc = #doc_msg]
                pub fn new(#args) -> Self {
                    Self {
                        #names
                        ..Default::default()
                    }
                }
            }
        }
        _ => TokenStream2::new(),
    }
}

fn construct_impl_sign(
    field_info: FieldInfo,
    for_self: bool,
) -> Option<(TokenStream2, TokenStream2)> {
    let FieldInfo {
        name,
        ty,
        strip_option,
        skip_setter,
        ..
    } = field_info;

    let method_name = quote::format_ident!("with_{}", name);
    let vis = for_self
        .then(|| TokenStream2::from_str("pub").unwrap())
        .unwrap_or_else(TokenStream2::new);

    if !skip_setter {
        let doc_msg = format!("Set the {} field of this struct.", name);
        Some(
            if let (true, Some(stripped_ty)) = (strip_option, extract_type_from_option(&ty)) {
                let impll = quote! {
                    #[doc = #doc_msg]
                    #vis fn #method_name(self, #name: impl Into<#stripped_ty>) -> Self {
                        let mut new = self;
                        new.#name = Some(#name.into());
                        new
                    }
                };
                let sign = quote! {
                    #[doc = #doc_msg]
                    fn #method_name(self, #name: impl Into<#stripped_ty>) -> Self;
                };
                (impll, sign)
            } else {
                let impll = quote! {
                    #[doc = #doc_msg]
                    #vis fn #method_name(self, #name: impl Into<#ty>) -> Self {
                        let mut new = self;
                        new.#name = #name.into();
                        new
                    }
                };
                let sign = quote! {
                    #[doc = #doc_msg]
                    fn #method_name(self, #name: impl Into<#ty>) -> Self;
                };
                (impll, sign)
            },
        )
    } else {
        None
    }
}

fn extract_type_from_option(ty: &Type) -> Option<Type> {
    fn path_is_option(path: &Path) -> bool {
        path.segments.last().unwrap().ident == "Option"
    }

    let t = match ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            if !path_is_option(&typepath.path) {
                return None;
            }
            // Get the first segment of the path (there is only one, in fact: "Option"):
            let type_params = &typepath.path.segments.last().unwrap().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => panic!("TODO: error handling {}:{}", file!(), line!()),
            };
            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => ty.clone(),
                _ => panic!("TODO: error handling {}:{}", file!(), line!()),
            }
        }
        _ => panic!("TODO: error handling {}:{}", file!(), line!()),
    };

    Some(t)
}
