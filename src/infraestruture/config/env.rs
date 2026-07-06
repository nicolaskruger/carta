pub struct Env {
    pub database_url: String,
    pub root_name: String,
    pub root_password: String,
}
#[derive(Debug)]
pub enum EnvError {
    NoDatabaseUrl,
    NoRootName,
    NoRootPassword,
}

fn env_decoder(key: &str, err: EnvError) -> Result<String, EnvError> {
    std::env::var(key).map_err(|_| err)
}

pub fn load_env() -> Result<Env, EnvError> {
    dotenvy::dotenv().ok();

    let env = Env {
        database_url: env_decoder("DATABASE_URL", EnvError::NoDatabaseUrl)?,
        root_name: env_decoder("ROOT_NAME", EnvError::NoRootName)?,
        root_password: env_decoder("ROOT_PASSWORD", EnvError::NoRootPassword)?,
    };

    Ok(env)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore = "reason"]
    fn load_env_test() -> Result<(), EnvError> {
        let _env = load_env()?;

        assert!(_env.database_url.len() > 1);
        assert!(_env.root_name.len() > 1);
        assert!(_env.root_password.len() > 1);

        Ok(())
    }
}
