use rocksdb::{Options, DB};
use std::path::PathBuf;

use thiserror::Error;

/// Database error
#[derive(Debug, Error)]
pub enum DbError {
    #[error("Invalid bulk write kv lengths, must be equal")]
    InvalidBulkLen,
    #[error("Cannot use unopened database")]
    Unopened,
    #[error(transparent)]
    Database(#[from] rocksdb::Error),
    #[error("{0}")]
    Other(String),
}

impl From<DbError> for String {
    fn from(e: DbError) -> Self {
        e.to_string()
    }
}

pub struct Store {
    db: DB,
    path: PathBuf,
}

impl Store {
    pub fn open(path: PathBuf, opts: Options) -> Result<Store, DbError> {
        let db = DB::open(&opts, &path)?;
        Ok(Self { db, path })
    }

    pub fn open_default(path: PathBuf) -> Result<Store, DbError> {
        let db = DB::open_default(&path)?;
        Ok(Self { db, path })
    }

    pub fn put<K, V>(&self, k: K, v: V) -> Result<(), DbError>
    where
        K: AsRef<[u8]>,
        V: AsRef<[u8]>,
    {
        Ok(self.db.put(k, v)?)
    }

    pub fn get<K>(&self, key: K) -> Result<Option<Vec<u8>>, DbError>
    where
        K: AsRef<[u8]>,
    {
        self.db.get(key).map_err(DbError::from)
    }

    pub fn delete<K>(&self, key: K) -> Result<(), DbError>
    where
        K: AsRef<[u8]>,
    {
        Ok(self.db.delete(key)?)
    }

    pub fn close(&self) -> Result<(), DbError> {
        Ok(DB::destroy(&Options::default(), &self.path)?)
    }
}
