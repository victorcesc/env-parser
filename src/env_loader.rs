use std::env;
use crate::{config::Config, errors::ConfigError};
use std::path::Path;


pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    dotenvy::from_path(path).map_err(|e| {
    ConfigError::LoadError(format!("failed to load config: {}", e))
})?;
    let port = get_u16("PORT")?;
    // let debug = get_bool("DEBUG")?;
    let host = get_string("HOST")?;
    let database_url = get_string("DB_URI")?;
    Ok(Config { port, host, database_url })
}

// se o valor não for encontrado, retorna um erro de variável de ambiente faltando
fn get_string(key: &str) -> Result<String, ConfigError> {
    env::var(key).map_err(|e| ConfigError::MissingEnvironmentVariable(format!("{}: {}", key, e)))
}

// se o valor não for um número inteiro, retorna um erro de valor inválido
fn get_u16(key: &str) -> Result<u16, ConfigError> {
    let value = get_string(key)?;
    value.parse::<u16>().map_err(|_e| ConfigError::InvalidValue(key.to_string(), "invalid integer".into()))
}

// // se o valor não for um booleano, retorna um erro de valor inválido
// fn get_bool(key: &str) -> Result<bool, ConfigError> {
//     let value = get_string(key)?;
//     value.parse::<bool>().map_err(|_e| ConfigError::InvalidValue(key.to_string(), "invalid boolean".into()))
// }