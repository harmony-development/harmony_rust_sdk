use proc_macro::TokenStream;

mod impl_call;
mod impl_into_request;
mod impl_self_builder;

#[proc_macro_attribute]
pub fn into_request(args: TokenStream, input: TokenStream) -> TokenStream {
    impl_into_request::impl_into_request(args, input)
}

#[proc_macro_derive(builder, attributes(builder))]
pub fn self_builder(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_self_builder::self_builder(&input, Default::default())
}

#[proc_macro_derive(self_builder, attributes(builder))]
pub fn self_builder_impl(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_self_builder::self_builder(
        &input,
        impl_self_builder::Config {
            for_self: true,
            ..Default::default()
        },
    )
}

// HACK: remove this when prost-build gains "removing" type attrs per match
#[proc_macro_attribute]
pub fn self_builder_with_new(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let is_disc_enum = {
        if let syn::Data::Enum(data) = &input.data {
            data.variants.iter().any(|v| v.discriminant.is_some())
        } else {
            false
        }
    };

    let self_impl: proc_macro2::TokenStream = impl_self_builder::self_builder(
        &input,
        impl_self_builder::Config {
            for_self: true,
            ..Default::default()
        },
    )
    .into();

    if is_disc_enum {
        (quote::quote! {
            #input

            #self_impl
        })
        .into()
    } else {
        (quote::quote! {
            #[derive(derive_new::new)]
            #input

            #self_impl
        })
        .into()
    }
}

#[proc_macro_derive(self_builder_no_option, attributes(builder))]
pub fn self_builder_impl_strip_option(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    impl_self_builder::self_builder(
        &input,
        impl_self_builder::Config {
            for_self: true,
            strip_option: true,
        },
    )
}

#[proc_macro]
pub fn impl_into_req_from(input: TokenStream) -> TokenStream {
    impl_into_request::impl_into_req_from(input)
}

#[proc_macro]
pub fn impl_call(input: TokenStream) -> TokenStream {
    impl_call::impl_call(input)
}

#[proc_macro_attribute]
pub fn impl_call_action(args: TokenStream, input: TokenStream) -> TokenStream {
    impl_call::impl_call_action(args, input)
}

#[proc_macro_attribute]
pub fn impl_call_req(args: TokenStream, input: TokenStream) -> TokenStream {
    impl_call::impl_call_req(args, input)
}
