use tonic::{Request, Response, Status, transport::Server};

use user::user_service_server::{UserService, UserServiceServer};
use user::{AuthRequest, AuthResponse};

pub mod user {
    tonic::include_proto!("user");
}
