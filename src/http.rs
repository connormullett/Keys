use std::net::SocketAddr;
use std::{convert::Infallible, sync::Arc};

use hyper::Server;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};

use crate::{db::Store, Config};

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World")))
}

pub async fn start_server(config: &Config, _db: Arc<Store>) {
    println!("starting http server..");

    let make_service = make_service_fn(|_| {
        let service = service_fn(move |req| handle(req));

        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("Server error {e}")
    }
}
