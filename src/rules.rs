use crate::{config::Config, errors::ConfigError};
use url::Url;


pub fn validate(config: &Config) -> Result<(), ConfigError> {
    if config.port < 1024 {
        return Err(ConfigError::InvalidValue("PORT".into(), "port must be greater than 1024".into()));
    }
    Url::parse(&config.database_url).map_err(|_e| ConfigError::InvalidValue("DATABASE_URL".into(), "invalid database URL".into()))?;
    if config.host.is_empty() {
        return Err(ConfigError::InvalidValue("HOST".into(), "host cannot be empty".into()));
    }
    Ok(())
}