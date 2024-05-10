use std::pin::Pin;

use hyper::{Body, Error, Request, Response};

mod server;

#[derive(Clone)]
struct HandlerImpl;

impl server::Handler for HandlerImpl {
    fn handle(&self, req: Request<Body>) -> Pin<Box<dyn std::future::Future<Output = Result<Response<Body>, Error>> + Send>> {
        Box::pin(async move {
            println!("request: {:?} {:?}!", req.method(), req.uri());
            match req.uri().path() {
                "/games" => Ok(Response::new(Body::from("These are your games"))),
                _ => Ok(Response::builder()
                    .status(404)
                    .body(Body::from("Not Found"))
                    .unwrap()),
            }
        })
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Err(err) = server::start(8080, HandlerImpl{}).await {
        println!("Error starting server: {:?}", err);
        return Err(err);
    }

    Ok(())
}
