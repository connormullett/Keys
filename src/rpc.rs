use std::str;
use std::sync::Arc;

use futures::future::{self, Ready};
use tarpc::context;

use crate::db::Store;

#[tarpc::service]
trait StoreService {
    async fn get(key: String) -> String;
    async fn put(key: String, value: String) -> String;
    async fn delete(key: String) -> String;
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

    type PutFut = Ready<String>;

    fn put(self, _: context::Context, key: String, value: String) -> Self::PutFut {
        let value = match self.db.put(key, value) {
            Ok(_) => "OK".to_string(),
            Err(e) => e.to_string(),
        };
        future::ready(value)
    }

    type DeleteFut = Ready<String>;

    fn delete(self, _: context::Context, key: String) -> Self::DeleteFut {
        let value = match self.db.delete(key) {
            Ok(_) => "OK".to_string(),
            Err(e) => e.to_string(),
        };
        future::ready(value)
    }
}
