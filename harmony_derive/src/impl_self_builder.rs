use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, Data, DeriveInput, Field, GenericArgument, Ident, Path, PathArguments, Type,
};

pub(crate) fn self_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let trait_name = format_ident!("{}SelfBuilder", name);
    let fields = match &input.data {
        Data::Struct(s) => s.fields.iter().map(FieldInfo::from),
        _ => panic!("#[self_builder] must be implemented on a struct."),
    };
    let impls_signs = fields.flat_map(construct_impl_sign).collect::<Vec<_>>();

    let mut impls = Vec::with_capacity(impls_signs.len());
    let mut signs = Vec::with_capacity(impls_signs.len());
    for (impll, sign) in impls_signs {
        impls.push(impll);
        signs.push(sign);
    }

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

fn construct_impl_sign(field_info: FieldInfo) -> Option<(TokenStream2, TokenStream2)> {
    let FieldInfo {
        name,
        ty,
        strip_option,
        skip_setter,
    } = field_info;

    if !skip_setter {
        let doc_msg = format!("Set the {} field of this struct.", name);
        Some(if strip_option {
            let stripped_ty: Type = extract_type_from_option(&ty);

            let impll = quote! {
                #[doc = #doc_msg]
                fn #name(self, #name: impl Into<#stripped_ty>) -> Self {
                    let mut new = self;
                    new.#name = Some(#name.into());
                    new
                }
            };
            let sign = quote! {
                #[doc = #doc_msg]
                fn #name(self, #name: impl Into<#stripped_ty>) -> Self;
            };
            (impll, sign)
        } else {
            let impll = quote! {
                #[doc = #doc_msg]
                fn #name(self, #name: impl Into<#ty>) -> Self {
                    let mut new = self;
                    new.#name = #name.into();
                    new
                }
            };
            let sign = quote! {
                #[doc = #doc_msg]
                fn #name(self, #name: impl Into<#ty>) -> Self;
            };
            (impll, sign)
        })
    } else {
        None
    }
}

fn extract_type_from_option(ty: &Type) -> Type {
    fn path_is_option(path: &Path) -> bool {
        path.segments.last().unwrap().ident == "Option"
    }

    match ty {
        Type::Path(typepath) if typepath.qself.is_none() && path_is_option(&typepath.path) => {
            // Get the first segment of the path (there is only one, in fact: "Option"):
            let type_params = &typepath.path.segments.first().unwrap().arguments;
            // It should have only on angle-bracketed param ("<String>"):
            let generic_arg = match type_params {
                PathArguments::AngleBracketed(params) => params.args.first().unwrap(),
                _ => panic!("TODO: error handling"),
            };
            // This argument must be a type:
            match generic_arg {
                GenericArgument::Type(ty) => ty.clone(),
                _ => panic!("TODO: error handling"),
            }
        }
        _ => panic!("TODO: error handling"),
    }
}
