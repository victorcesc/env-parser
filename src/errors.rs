use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // #[error("Failed to parse environment variable {0}: {1}")]
    // EnvParseError(String, String),

    #[error("obrigatory environment variable {0} not found")]
    MissingEnvironmentVariable(String),

    #[error("invalid value for {0}: {1}")]
    InvalidValue(String, String),

    #[error("Failed to load config: {0}")]
    LoadError(String),
}