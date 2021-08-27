use proc_macro::TokenStream;

mod impl_into_request;
mod impl_self_builder;

#[proc_macro_attribute]
pub fn into_request(args: TokenStream, input: TokenStream) -> TokenStream {
    impl_into_request::impl_into_request(args, input)
}

#[proc_macro_derive(builder, attributes(builder))]
pub fn self_builder(input: TokenStream) -> TokenStream {
    impl_self_builder::self_builder(input)
}
