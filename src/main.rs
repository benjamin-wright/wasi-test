use tokio::net::TcpListener;
use std::{net::SocketAddr, time::Duration};

use hyper::server::conn::Http;
use hyper::{Body, Error, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};

async fn handler(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("request: {:?} {:?}!", req.method(), req.uri());
    match req.uri().path() {
        "/games" => games(req).await,
        "/kill" => {
            tokio::spawn(async {
                tokio::time::sleep(Duration::from_secs(1)).await;
                std::process::exit(0);
            });
            Ok(Response::new(Body::from("Server is shutting down")))
        },
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

async fn wait_for_kill_signal() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8081));

    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => listener,
        Err(err) => {
            println!("Failed to bind kill signal listener: {:?}", err);
            return;
        }
    };

    println!("Listening for kill signal on http://{}", addr);

    loop {
        let (stream, _) = match listener.accept().await {
            Ok((stream, _)) => (stream, ()),
            Err(err) => {
                println!("Failed to accept kill signal: {:?}", err);
                return;
            }
        };

        match Http::new()
            .serve_connection(stream, service_fn(|_| async {
                Ok::<_, Error>(Response::new(Body::from("Shutting down")))
            })).await {
                Ok(_) => {
                    break;
                },
                Err(err) => println!("Failed to response to kill signal response: {:?}", err),
            };
    }

    println!("Kill signal received");
}
