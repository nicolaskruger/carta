pub struct Env {
    databese_url: String,
}

pub enum EnvError {
    NO_DATABASE_URL,
}

fn env_decoder(key: String, err: EnvError) -> Result<String, EnvError> {
    std::env::var(key).map_err(|_| err)
}

pub fn load_env() -> Result<Env, EnvError> {
    dotenvy::dotenv().ok();

    let _env = Env {
        databese_url: env_decoder("DATABESE_URL".into(), EnvError::NO_DATABASE_URL)?,
    };

    Ok(_env)
}
