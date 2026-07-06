use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn db_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[derive(Debug)]
pub enum MigrationError {
    LoadEnv,
    SqlPool,
    RunMigration,
}

pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}
