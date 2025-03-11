use dotenv::dotenv;
use log::info;
use serde::Deserialize;
use std::env;

/// Configuration for the application
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    /// Root directory to monitor for log files
    pub root_dir: String,

    /// MongoDB connection URI
    pub mongodb_uri: String,

    /// MongoDB database name
    pub database: String,

    /// MongoDB collection name
    pub collection: String,

    /// Polling interval in milliseconds for file tailing
    pub polling_interval_ms: u64,
}

/// Load application configuration from environment variables
///
/// Uses dotenv for local development and falls back to defaults
/// for some parameters if not specified
pub fn load_config() -> Result<Config, String> {
    dotenv().ok();

    let root_dir =
        env::var("ROOT_DIR").map_err(|_| "ROOT_DIR environment variable not set".to_string())?;

    let mongodb_uri = env::var("MONGODB_URI")
        .map_err(|_| "MONGODB_URI environment variable not set".to_string())?;

    let database = env::var("DATABASE").unwrap_or_else(|_| {
        info!("DATABASE not set, using default");
        "trades_db".to_string()
    });

    let collection = env::var("COLLECTION").unwrap_or_else(|_| {
        info!("COLLECTION not set, using default");
        "trades".to_string()
    });

    let polling_interval_ms = env::var("POLLING_INTERVAL_MS")
        .unwrap_or_else(|_| {
            info!("POLLING_INTERVAL_MS not set, using default");
            "500".to_string()
        })
        .parse::<u64>()
        .map_err(|_| "POLLING_INTERVAL_MS must be a valid integer".to_string())?;

    Ok(Config {
        root_dir,
        mongodb_uri,
        database,
        collection,
        polling_interval_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        // This test assumes ROOT_DIR and MONGODB_URI are set
        // or will be skipped
        if env::var("ROOT_DIR").is_err() || env::var("MONGODB_URI").is_err() {
            return;
        }

        let config = load_config().unwrap();
        assert_eq!(
            config.database, "trades_db",
            "Default database should be 'trades_db'"
        );
        assert_eq!(
            config.collection, "trades",
            "Default collection should be 'trades'"
        );
        assert_eq!(
            config.polling_interval_ms, 500,
            "Default polling interval should be 500ms"
        );
    }
}
