use config::{ Config, ConfigError, File };
use dotenvy::dotenv;
use serde::Deserialize;
use diesel::{ pg::PgConnection, Connection };

#[derive(Deserialize)]
pub struct ServerConfig {
    scheme: String,
    host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    url: String,
}

impl DatabaseConfig {
    pub fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).unwrap_or_else(|_|
            panic!("Error connecting to {}", database_url)
        )
    }
}

#[derive(Deserialize)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());
        let config = std::env
            ::current_dir()
            .expect("Can't access current directory")
            .join("config");

        let cfg = Config::builder()
            .add_source(File::from(config.join("default.toml")))
            .add_source(File::from(config.join(format!("{}.toml", run_env))).required(false))
            .build()?;
        cfg.try_deserialize()
    }
}
