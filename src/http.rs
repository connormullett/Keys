use std::net::SocketAddr;
use std::str;
use std::{convert::Infallible, sync::Arc};

use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response,
};
use hyper::{Method, Server};

use crate::{db::Store, Config};

#[derive(Clone)]
struct ServerContext {
    pub db: Arc<Store>,
}

fn http_get(context: ServerContext, key: String) -> String {
    match context.db.get(key) {
        Ok(value) => {
            let value = value.unwrap_or(vec![]);
            str::from_utf8(&value).expect("fixme").to_string()
        }
        Err(_) => String::from("(nil)"),
    }
}

async fn handle(context: ServerContext, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let (parts, _body) = req.into_parts();
    match parts.method {
        Method::GET => {
            let key = parts.uri.path().split("/").last().unwrap();
            let value = http_get(context, key.to_owned());
            Ok(Response::new(Body::from(value)))
        }
        Method::PUT => todo!(),
        Method::DELETE => todo!(),
        Method::OPTIONS => todo!(),
        _ => todo!(),
    }
}

pub async fn start_server(config: &Config, db: Arc<Store>) {
    println!("starting http server..");

    let context = ServerContext { db };

    let make_service = make_service_fn(|_| {
        let context = context.clone();
        let service = service_fn(move |req| handle(context.clone(), req));

        async move { Ok::<_, Infallible>(service) }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));

    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("Server error {e}")
    }
}
