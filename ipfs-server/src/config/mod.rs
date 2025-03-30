mod env;

use crate::error::Error;
use config::Config;
use serde::Deserialize;

use self::env::ENV;

const CONFIG_FILE_PATH: &str = "./config/main";
const CONFIG_FILE_PATH_PREFIX: &str = "./config/";

lazy_static::lazy_static! {
    pub static ref CFG: IPFSConfig = {
        parse().unwrap()
    };
}

#[derive(Clone, Deserialize, Default)]
pub struct IPFSConfig {
    pub db: ConfigDB,
    pub web: ConfigWeb,
    pub contract: ConfigContract,
    pub arweave: Option<ConfigArwave>,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigDB {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigWeb {
    pub listen: String,
    pub port: u16,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigContract {
    pub contract: String,
}

#[derive(Clone, Deserialize, Default)]
pub struct ConfigArwave {
    pub jwt: String,
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub enum ConfigCategory {
    File,
}

impl Default for ConfigCategory {
    fn default() -> Self {
        Self::File
    }
}

/// Fetch and parse runtime ENV.
pub fn app_env() -> ENV {
    std::env::var("IPFS_SERVER_ENV")
        .unwrap_or_else(|_| "development".into())
        .into()
}

/// Parse config from local file or ENV.
pub fn parse() -> Result<IPFSConfig, Error> {
    let s = Config::builder()
        // Default
        .add_source(config::File::with_name(CONFIG_FILE_PATH).required(false))
        // app-env-based config
        .add_source(
            config::File::with_name(&format!("{}{}.toml", CONFIG_FILE_PATH_PREFIX, app_env()))
                .required(false),
        )
        // runtime-ENV-based config
        .add_source(
            config::Environment::with_prefix("IPFS")
                .separator("__")
                .ignore_empty(true),
        )
        .build()?;

    s.try_deserialize().map_err(|e| e.into())
}

impl IPFSConfig {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.db.username, self.db.password, self.db.host, self.db.port, self.db.db,
        )
    }
}
