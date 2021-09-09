use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Field, GenericArgument, Ident, Path, PathArguments, Type,
};

pub(crate) fn self_builder(input: TokenStream, for_self: bool, strip_option: bool) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let trait_name = format_ident!("{}SelfBuilder", name);
    let fields = match &input.data {
        Data::Struct(s) => s.fields.iter().map(FieldInfo::from).map(|mut f| {
            if strip_option {
                f.strip_option = true;
            }
            f
        }),
        _ => return TokenStream::new(),
    };
    let impls_signs = fields
        .flat_map(|f| construct_impl_sign(f, for_self))
        .collect::<Vec<_>>();

    let mut impls = Vec::with_capacity(impls_signs.len());
    let mut signs = Vec::with_capacity(impls_signs.len());
    for (impll, sign) in impls_signs {
        impls.push(impll);
        signs.push(sign);
    }

    if for_self {
        (quote! {
            impl #name {
                #(
                    #impls
                )*
            }
        })
        .into()
    } else {
        (quote! {
            /// Builder trait for #name.
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
    strip_option: bool,
    skip_setter: bool,
}

impl From<&Field> for FieldInfo {
    fn from(field: &Field) -> Self {
        let mut strip_option = false;
        let mut skip_setter = false;
        for attr in field.attrs.iter() {
            match attr.tokens.to_string().as_str() {
                "builder(setter(strip_option))" => strip_option = true,
                "builder(setter(skip))" => skip_setter = true,
                _ => {}
            }
        }

        Self {
            name: field.ident.as_ref().expect("Expected a name").clone(),
            ty: field.ty.clone(),
            strip_option,
            skip_setter,
        }
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
