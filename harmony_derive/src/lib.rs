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
    impl_self_builder::self_builder(input, false, false)
}

#[proc_macro_derive(self_builder, attributes(builder))]
pub fn self_builder_impl(input: TokenStream) -> TokenStream {
    impl_self_builder::self_builder(input, true, false)
}

#[proc_macro_derive(self_builder_no_option, attributes(builder))]
pub fn self_builder_impl_strip_option(input: TokenStream) -> TokenStream {
    impl_self_builder::self_builder(input, true, true)
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
