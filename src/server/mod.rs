use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use hyper::{Error, Request, Response, Body, Server};

use hyper::service::{make_service_fn, service_fn};

mod wait_for_kill;

pub trait Handler: Send {
    fn handle(&self, req: Request<hyper::Body>) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send>>;
}

pub async fn start(port: u16, handler: impl Handler + Send + Sync + Clone + 'static) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let make_svc = make_service_fn(move |_| {
        let handler = handler.clone();
        async {
            Ok::<_, Error>(service_fn(move |req| {
                let handler = handler.clone();
                async move {
                    let res = handler.handle(req);
                    let res = res.await?;
                    Ok::<Response<Body>, Error>(res)
                }
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(wait_for_kill::wait_for_kill_signal());

    if let Err(err) = graceful.await {
        println!("Error shutting down server: {:?}", err);
    } else {
        println!("Server shut down gracefully");
    }

    Ok(())
}