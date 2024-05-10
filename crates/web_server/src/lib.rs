use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// Check out these: https://earthly.dev/blog/rust-macros/
// https://github.com/rwf2/Rocket/blob/master/examples/hello/src/main.rs

#[proc_macro_attribute]
pub fn server(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    TokenStream::from(quote!(
        use std::pin::Pin;
        use hyper::{Body, Error, Request, Response};

        use web_helpers;

        #[derive(Clone)]
        struct HandlerImpl;

        #input

        impl web_helpers::Handler for HandlerImpl {
            fn handle(&self, req: Request<Body>) -> Pin<Box<dyn std::future::Future<Output = Result<Response<Body>, Error>> + Send>> {
                Box::pin(async move {
                    println!("request: {:?} {:?}!", req.method(), req.uri());
                    handler(req).await
                })
            }
        }

        #[tokio::main(flavor = "current_thread")]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            if let Err(err) = web_helpers::start(8080, HandlerImpl{}).await {
                println!("Error starting server: {:?}", err);
                return Err(err);
            }

            Ok(())
        }
    ))
}