use std::{net::SocketAddr, sync::Arc};

use crate::{db::Store, Config};

pub async fn start_server(config: &Config, db: Arc<Store>) {
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("starting http server..")
}
