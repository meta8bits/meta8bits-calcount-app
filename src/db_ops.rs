//! Database operations; squirrel code lives here.

use super::{models, stripe};
use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query_as,
};

pub async fn create_pg_pool() -> Result<sqlx::Pool<sqlx::Postgres>> {
    let db_url = &std::env::var("DATABASE_URL")
        .expect("database url to be defined in the environment")[..];

    Ok(PgPoolOptions::new()
        // Postgres default max connections is 100, and we'll take 'em
        // https://www.postgresql.org/docs/current/runtime-config-connection.html
        .max_connections(80)
        .connect(db_url)
        .await?)
}

#[async_trait]
pub trait DbModel<GetQuery, ListQuery>: Sync + Send {
    /// Get exactly one object from the database, matching the query. WIll
    /// return an error variant if the item does not exist.
    async fn get(db: &PgPool, query: &GetQuery) -> Result<Self>
    where
        Self: Sized;
    /// Get a set of objects from the database, matching the contents of the
    /// list query type.
    async fn list(db: &PgPool, query: &ListQuery) -> Result<Vec<Self>>
    where
        Self: Sized;
    /// Persist the object to the database
    async fn save(&self, db: &PgPool) -> Result<()>;
    /// Delete the record from the databse, which could of course cascade
  