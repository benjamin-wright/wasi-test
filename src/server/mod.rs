use std::future::Future;
use std::net::SocketAddr;
use hyper::server::conn::{AddrIncoming, Http};
use hyper::{Error, Request, Server};

use hyper::service::{make_service_fn, service_fn};

pub mod wait_for_kill;

// async fn handle(req: Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
//     println!("request: {:?} {:?}!", req.method(), req.uri());
//     match req.uri().path() {
//         "/games" => Ok(hyper::Response::new(hyper::Body::from("These are your games"))),
//         "/kill" => {
//             tokio::spawn(async {
//                 tokio::time::sleep(std::time::Duration::from_secs(1)).await;
//                 std::process::exit(0);
//             });
//             Ok(hyper::Response::new(hyper::Body::from("Server is shutting down")))
//         },
//         _ => Ok(hyper::Response::builder()
//             .status(404)
//             .body(hyper::Body::from("Not Found"))
//             .unwrap()),
//     }
// }

pub trait Handler {
    async fn handle(&self, req: Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error>;
}

pub async fn start(port: u16, handler: impl Handler + Send + Sync) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(|_req: Request<hyper::Body>| async {
            handler.handle(_req).await
        }))
    }));
    println!("Listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(wait_for_kill::wait_for_kill_signal());

    if let Err(err) = graceful.await {
        println!("Error shutting down server: {:?}", err);
    } else {
        println!("Server shut down gracefully");
    }

    Ok(())
}