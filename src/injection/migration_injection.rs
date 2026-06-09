use crate::config::{
    db_pool::{MigrationError, db_pool, run_migrations},
    env::load_env,
};

pub async fn migrate() -> Result<(), MigrationError> {
    let env = load_env().map_err(|_| MigrationError::LoadEnv)?;

    let pool = db_pool(env.database_url)
        .await
        .map_err(|_| MigrationError::SqlPool)?;

    run_migrations(&pool)
        .await
        .map_err(|_| MigrationError::RunMigration)?;

    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    #[ignore = "reason"]
    #[should_panic(expected = "something go wrong")]
    async fn migrate_test() {
        migrate().await.expect("something go wrong");
    }
}
