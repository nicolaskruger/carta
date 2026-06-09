use carta::injection::migration_injection::migrate;

#[derive(Debug)]
enum MainError {
    Migration,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    migrate().await.map_err(|_| MainError::Migration)?;

    Ok(())
}
