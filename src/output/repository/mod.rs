mod error;
mod identity;

pub use error::{Error, Result};

use crate::input::config::RepositoryConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type Db = Pool<Postgres>;

// region: -- SqlRepository
pub struct SqlRepository {
  connection: Db,
}

impl SqlRepository {
  pub async fn create(config: &RepositoryConfig) -> Result<SqlRepository> {
    info!("database connection is creating");
    let pool: Pool<Postgres> = PgPoolOptions::new()
      .max_connections(config.pool)
      .connect(&build_db_url(config))
      .await
      .map_err(|er| Error::FailToCreatePool {
        reason: er.to_string(),
      })?;

    Ok(SqlRepository { connection: pool })
  }
}
// endregion: -- SqlRepository

// region: -- private func
fn build_db_url(config: &RepositoryConfig) -> String {
  format!(
    "postgres://{}:{}@{}:{}/{}",
    &config.user, &config.password, &config.host, &config.port, &config.database
  )
}
// endregion: -- private func

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_build_db_url_ok() -> Result<()> {
    let config = &RepositoryConfig {
      host: "1.1.1.1".to_string(),
      port: 88,
      user: "user".to_string(),
      password: "password".to_string(),
      database: "database".to_string(),
      pool: 123,
    };

    let actual_db_url = build_db_url(&config);
    let expected_db_url = "postgres://user:password@1.1.1.1:88/database".to_string();

    assert_eq!(expected_db_url, actual_db_url);

    Ok(())
  }
}
