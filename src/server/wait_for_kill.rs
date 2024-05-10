use std::{io::Error, net::SocketAddr};

use hyper::{server::conn::Http, service::service_fn, Body, Response};
use tokio::net::TcpListener;

pub async fn wait_for_kill_signal() {
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
