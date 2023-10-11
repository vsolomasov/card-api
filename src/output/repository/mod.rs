mod error;
mod identity;

use error::{Error, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub struct SqlRepository {
  connection: Db,
}

impl SqlRepository {
  pub async fn create(db_url: &str, max_connections: u32) -> Result<SqlRepository> {
    let pool = PgPoolOptions::new()
      .max_connections(max_connections)
      .connect(db_url)
      .await
      .map_err(|er| Error::FailToCreatePool {
        reason: er.to_string(),
      })?;

    Ok(SqlRepository { connection: pool })
  }
}
