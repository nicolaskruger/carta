use tonic::transport::Server;

use crate::infraestruture::server::user_server::user_server_config;

pub async fn grpc_config() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(user_server_config()?)
        .serve(addr)
        .await?;

    Ok(())
}
