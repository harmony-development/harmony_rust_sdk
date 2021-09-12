use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub(crate) fn impl_call(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut split = input.split(',').map(str::trim);
    let service = Ident::new(split.next().unwrap(), Span::call_site());
    let method = Ident::new(split.next().unwrap(), Span::call_site());
    let req = Ident::new(split.next().unwrap(), Span::call_site());
    let resp = Ident::new(split.next().unwrap(), Span::call_site());

    let endpoint_path = format!("/protocol.{}.v1/{}", service, req.to_string().trim_end_matches("Request"));

    (quote! {
        #[hrpc::async_trait]
        impl crate::client::CallRequest for #req {
            type Response = #resp;

            const ENDPOINT_PATH: &'static str = #endpoint_path;

            async fn call_with(self, client: &crate::client::Client) -> crate::client::error::ClientResult<Self::Response> {
                client. #service () .await. #method (self) .await.map_err(Into::into)
            }
        }
    })
    .into()
}

pub(crate) fn impl_call_action(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let service: TokenStream2 = args.into();
    let action = input.ident.to_string();

    let method = naive_snake_case(&action);

    let inputt = format!("{}, {}, {}, {}Response", service, method, action, action);
    let stream: TokenStream2 = impl_call(inputt.parse().unwrap()).into();
    (quote! {
        #input
        #stream
    })
    .into()
}

pub(crate) fn impl_call_req(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let service: TokenStream2 = args.into();
    let req = input.ident.to_string();

    match req.strip_suffix("Request") {
        Some(action) if req != "Request" && !req.starts_with("Stream") => {
            let method = naive_snake_case(action);

            let inputt = format!(
                "{}, {}, {}Request, {}Response",
                service, method, action, action
            );
            let stream: TokenStream2 = impl_call(inputt.parse().unwrap()).into();
            (quote! {
                #input
                #stream
            })
            .into()
        }
        _ => (quote! { #input }).into(),
    }
}

fn naive_snake_case(name: &str) -> String {
    let mut s = String::new();
    let mut it = name.chars().peekable();

    while let Some(x) = it.next() {
        s.push(x.to_ascii_lowercase());
        if let Some(y) = it.peek() {
            if y.is_uppercase() {
                s.push('_');
            }
        }
    }

    s
}
