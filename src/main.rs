use web_server::server;

#[server]
async fn handler(req: Request<Body>) -> Result<Response<Body>, Error> {
    match req.uri().path() {
        "/games" => Ok(Response::new(Body::from("These are your games"))),
        _ => Ok(Response::builder()
            .status(404)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}
