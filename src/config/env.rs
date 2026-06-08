pub struct Env {
    pub databese_url: String,
}
#[derive(Debug)]
pub enum EnvError {
    NoDatabaseUrl,
}

fn env_decoder(key: String, err: EnvError) -> Result<String, EnvError> {
    std::env::var(key).map_err(|_| err)
}

pub fn load_env() -> Result<Env, EnvError> {
    dotenvy::dotenv().ok();

    let _env = Env {
        databese_url: env_decoder("DATABESE_URL".into(), EnvError::NoDatabaseUrl)?,
    };

    Ok(_env)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore = "reason"]
    fn load_env_test() -> Result<(), EnvError> {
        let _env = load_env()?;

        assert!(_env.databese_url.len() > 1);

        Ok(())
    }
}
