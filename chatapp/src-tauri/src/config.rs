use config::{Config, ConfigError, File};
use serde::{Deserialize, Serialize};

use crate::utils::config_dir;

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub chat: String,
    pub notification: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}

impl AppConfig {
    pub fn try_new() -> Result<Self, ConfigError> {
        let config_file = config_dir().join("app.yaml");
        let config = Config::builder()
            .add_source(File::with_name("./src/fixtures/config.default.yaml"))
            .add_source(File::with_name(&config_file.to_string_lossy()).required(false))
            .build()?;
        config.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_config_should_work() {
        let config = AppConfig::try_new().unwrap();
        println!("{:?}", config);
    }
}
