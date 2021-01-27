use super::{Method, Service};
use crate::{generate_doc_comments, naive_snake_case};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate service for client.
///
/// This takes some `Service` and will generate a `TokenStream` that contains
/// a public module with the generated client.
pub fn generate<T: Service>(service: &T, proto_path: &str) -> TokenStream {
    let service_ident = quote::format_ident!("{}Client", service.name());
    let client_mod = quote::format_ident!("{}_client", naive_snake_case(&service.name()));
    let methods = generate_methods(service, proto_path);

    let service_doc = generate_doc_comments(service.comment());

    quote! {
        /// Generated client implementations.
        #[allow(dead_code, unused_imports)]
        pub mod #client_mod {
            use prost::Message;

            #service_doc
            #[derive(Debug, Clone)]
            pub struct #service_ident {
                inner: reqwest::Client,
                authorization: Option<String>,
                server: url::Url,
                buf: Vec<u8>,
            }

            impl #service_ident {
                pub fn new(inner: reqwest::Client, server: url::Url) -> Self {
                    if let "http" | "https" = server.scheme() {
                        Self {
                            inner,
                            server,
                            authorization: None,
                            buf: Vec::new(),
                        }
                    } else {
                        panic!("server base URL must have a scheme");
                    }
                }

                pub fn set_auth_token(&mut self, authorization: Option<String>) {
                    self.authorization = authorization;
                }

                fn make_request(&mut self, msg: impl prost::Message, path: &str) -> reqwest::Result<reqwest::Request> {
                    self.buf.reserve(msg.encoded_len() - self.buf.len());
                    self.buf.clear();
                    msg.encode(&mut self.buf).unwrap();

                    let mut req = self.inner.post(self.server.join(path).unwrap());
                    req = req.body(self.buf.drain(..).collect::<Vec<_>>());
                    if let Some(auth) = self.authorization.as_deref() {
                        req = req.bearer_auth(auth);
                    }

                    req.build()
                }

                #methods
            }
        }
    }
}

fn generate_methods<T: Service>(service: &T, proto_path: &str) -> TokenStream {
    let mut stream = TokenStream::new();

    for method in service.methods() {
        let path = format!(
            "/{}{}{}/{}",
            service.package(),
            if service.package().is_empty() {
                ""
            } else {
                "."
            },
            service.identifier(),
            method.identifier()
        );

        let mtd = match (method.client_streaming(), method.server_streaming()) {
            (false, false) => generate_unary(method, proto_path, path),
            _ => continue,
        };

        stream.extend(generate_doc_comments(method.comment()));
        stream.extend(mtd);
    }

    stream
}

fn generate_unary<T: Method>(method: &T, proto_path: &str, path: String) -> TokenStream {
    let ident = format_ident!("{}", method.name());
    let (request, response) = method.request_response_name(proto_path);

    quote! {
        pub async fn #ident(
            &mut self,
            request: impl Into<#request>,
        ) -> reqwest::Result<#response> {
            let req = self.make_request(request.into(), #path)?;
            let raw_resp = self.inner.execute(req).await?;
            let raw = raw_resp.error_for_status()?.bytes().await?;
            Ok(<#response>::decode(raw).unwrap())
        }
    }
}
