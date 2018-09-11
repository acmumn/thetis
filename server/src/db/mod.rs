mod schema;

use std::sync::Arc;

use diesel::{
    dsl::{exists, select},
    prelude::*,
    r2d2::{ConnectionManager, Pool, PoolError},
};
use futures::{
    future::{err, poll_fn, Either},
    prelude::*,
};
use tokio_threadpool::blocking;

use db::schema::{
    jwt_escrow, mail_member_subscriptions, mail_other_subscriptions, mail_send_queue,
    mail_unsubscribes, mailing_list_templates, mailing_lists, member_bans, member_payments,
    members, members_tag_join, tags,
};
use errors::DatabaseError;
use types::{MemberID, Tag};

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

    /// Gets the tags associated with a member.
    pub fn get_tags(
        &self,
        member: MemberID,
    ) -> impl Future<Item = Vec<Tag>, Error = DatabaseError> {
        self.async_query(move |conn| {
            members_tag_join::table
                .inner_join(tags::table)
                .filter(members_tag_join::member_id.eq(member))
                .select(tags::name)
                .get_results(conn)
                .map_err(|e| e.into())
        })
    }

    /// Checks if a member has a given tag.
    pub fn has_tag(
        &self,
        member: MemberID,
        tag: Tag,
    ) -> impl Future<Item = bool, Error = DatabaseError> {
        self.async_query(move |conn| {
            select(exists(
                members_tag_join::table
                    .inner_join(tags::table)
                    .filter(members_tag_join::member_id.eq(member))
                    .filter(tags::name.eq(tag.clone())),
            )).get_result(conn)
                .map_err(|e| e.into())
        })
    }

    /// Returns whether the member is banned.
    pub fn is_banned(&self, member: MemberID) -> impl Future<Item = bool, Error = DatabaseError> {
        self.async_query(move |conn| {
            unimplemented!();
        })
    }

    /// Returns whether the member is paid.
    pub fn is_paid(&self, member: MemberID) -> impl Future<Item = bool, Error = DatabaseError> {
        self.async_query(move |conn| {
            unimplemented!();
        })
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
