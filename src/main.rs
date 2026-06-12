use carta::config::{
    db_pool::{db_pool, run_migrations},
    env::load_env,
};

#[derive(Debug)]
enum MainError {
    Env,
    Migration,
    Pool,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let env = load_env().map_err(|_| MainError::Env)?;

    let pool = db_pool(&env.database_url)
        .await
        .map_err(|_| MainError::Pool)?;

    run_migrations(&pool)
        .await
        .map_err(|_| MainError::Migration)?;

    Ok(())
}
