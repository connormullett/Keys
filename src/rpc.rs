use std::str;
use std::sync::Arc;

use futures::future::{self, Ready};
use tarpc::context;

use crate::db::Store;

#[tarpc::service]
trait StoreService {
    async fn get(key: String) -> String;
}

#[derive(Clone)]
struct StoreServer {
    db: Arc<Store>,
}

impl StoreService for StoreServer {
    type GetFut = Ready<String>;

    fn get(self, _: context::Context, key: String) -> Self::GetFut {
        let value = match self.db.get(key) {
            Ok(val) => match val {
                Some(val) => str::from_utf8(&val).expect("fixme").to_string(),
                None => String::from("(nil)"),
            },
            Err(e) => e.to_string(),
        };
        future::ready(value)
    }
}
