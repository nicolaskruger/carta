use tonic::{Request, Response, Status};

use user::{AuthRequest, AuthResponse};

use user::user_service_server::{UserService, UserServiceServer};

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug, Default)]
pub struct UserServer {}

#[tonic::async_trait]
impl UserService for UserServer {
    async fn auth(&self, _: Request<AuthRequest>) -> Result<Response<AuthResponse>, Status> {
        let tmp = AuthResponse {
            token: "Bearer ".into(),
        };

        let tmp = Response::new(tmp);

        Ok(tmp)
    }
}

pub fn user_server_config() -> Result<UserServiceServer<UserServer>, Box<dyn std::error::Error>> {
    let user = UserServer::default();
    let user = UserServiceServer::new(user);
    Ok(user)
}
