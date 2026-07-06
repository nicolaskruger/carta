use reqwest::Client;
use serde::{Deserialize, Serialize};
use tonic::async_trait;

#[derive(Serialize)]
struct AcceptLoginRequest {
    subject: String,
    remember: bool,
    remember_for: u64,
}

#[derive(Deserialize)]
struct HydraAcceptResponse {
    redirect_to: String,
}
pub enum OAuthRepositoryErr {
    InternalServerError,
}

pub struct AcceptLoginInput {
    subject: String,
    login_challenge: String,
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait IOAuthRepository {
    async fn login_accept(&self, input: AcceptLoginInput) -> Result<String, OAuthRepositoryErr>;
}

pub struct HydraOAuthRepository {
    login_accept_uri: String,
}

#[async_trait]
impl IOAuthRepository for HydraOAuthRepository {
    async fn login_accept(&self, input: AcceptLoginInput) -> Result<String, OAuthRepositoryErr> {
        let put_uri = [
            self.login_accept_uri.to_string(),
            "admin/oauth2/auth/requests/consent/accept?consent_challenge=".to_string(),
            input.login_challenge.to_string(),
        ];

        let put_uri = put_uri.join("");

        let client = Client::new();
        let hydra_response = client
            .put(put_uri)
            .json(&AcceptLoginRequest {
                subject: input.subject,
                remember: true,
                remember_for: 3600,
            })
            .send()
            .await
            .map_err(|_| OAuthRepositoryErr::InternalServerError)?
            .json::<HydraAcceptResponse>()
            .await
            .map_err(|_| OAuthRepositoryErr::InternalServerError)?;

        Ok(hydra_response.redirect_to)
    }
}

impl HydraOAuthRepository {
    pub fn new(login_accept_uri: &str) -> Self {
        Self {
            login_accept_uri: login_accept_uri.to_string(),
        }
    }
}
