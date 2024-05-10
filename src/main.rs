use std::net::SocketAddr;

use hyper::{service::{make_service_fn, service_fn}, Body, Error, Request, Response, Server};

mod server;

use server::wait_for_kill::wait_for_kill_signal;


async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("request: {:?} {:?}!", req.method(), req.uri());
    match req.uri().path() {
        "/games" => games(req).await,
        _ => not_found().await,
    }
}

async fn games(_req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::builder()
        .status(200)
        .body(Body::from("These are your games"))
        .unwrap();

    Ok(response)
}

async fn not_found() -> Result<Response<Body>, hyper::Error> {
    let response = Response::builder()
        .status(404)
        .body(Body::from("Not Found"))
        .unwrap();

    Ok(response)
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    
    let server = Server::bind(&addr).serve(make_service_fn(|_| async {
        Ok::<_, Error>(service_fn(handler))
    }));
    println!("Listening on http://{}", addr);

    let graceful = server.with_graceful_shutdown(wait_for_kill_signal());

    if let Err(err) = graceful.await {
        println!("Error shutting down server: {:?}", err);
    } else {
        println!("Server shut down gracefully");
    }

    Ok(())
}
