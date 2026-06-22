use carta::config::{
    db_pool::{db_pool, run_migrations},
    env::load_env,
    grpc_config,
};

#[derive(Debug)]
enum MainError {
    Env,
    Migration,
    Pool,
    Grpc,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {

    tracing_subscriber::fmt::init();

    let env = load_env().map_err(|_| MainError::Env)?;

    let pool = db_pool(&env.database_url)
        .await
        .map_err(|_| MainError::Pool)?;

    run_migrations(&pool)
        .await
        .map_err(|_| MainError::Migration)?;

    grpc_config::grpc_config()
        .await
        .map_err(|_| MainError::Grpc)
}
