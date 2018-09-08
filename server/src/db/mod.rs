mod schema;

use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    MysqlConnection,
};
use futures::{
    future::{err, poll_fn, Either},
    prelude::*,
};
use tokio_threadpool::blocking;

/// A pool of connections to the database.
#[derive(Clone)]
pub struct DB {
    pool: Arc<Pool<ConnectionManager<MysqlConnection>>>,
}

impl DB {
    /// Connects to the database with at the given URL.
    pub fn connect(database_url: &str) -> Result<DB, PoolError> {
        let pool = Arc::new(Pool::new(ConnectionManager::new(database_url))?);
        Ok(DB { pool })
    }

    /// Performs a query "asynchronously" (but not really). Diesel currently does not support
    /// async/futures, so we use `tokio_threadpool::blocking` so the database operations don't
    /// block the thread.
    ///
    /// NOTE(remexre): In theory, this is now the bottleneck for most operations -- as I understand
    /// it, we can only have as many concurrent database operations as threads in the tokio thread
    /// pool, and it's not very hard for to exhaust the threadpool. If latency problems are noted,
    /// create the thread pool using `tokio_threadpool::Builder` to have
    /// `max_blocking < pool_size`.
    fn async_query<E, F, T>(&self, func: F) -> impl Future<Item = T, Error = E>
    where
        E: From<PoolError>,
        F: Fn(&MysqlConnection) -> ::std::result::Result<T, E>,
    {
        match self.pool.get() {
            Ok(conn) => Either::A(
                poll_fn(move || {
                    blocking(|| func(&*conn).map_err(|e| e.into())).map_err(|_| {
                        panic!("Database queries must be run inside a Tokio thread pool!")
                    })
                }).and_then(|r| r),
            ),
            Err(e) => Either::B(err(e.into())),
        }
    }
}
