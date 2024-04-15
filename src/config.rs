use std::io::Read;

use anyhow::Context;
use secrecy::Secret;
use serde::Deserialize;

// #[derive(Deserialize, Debug)]
// pub struct LogConfig {
//     pub level: String,
// }



#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    // pub log: LogConfig,
    pub secret_key: Secret<String>,
    pub domain_url: String,
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {

        match envy::from_env::<Config>() {
            Ok(config) => Ok(config),
            Err(e) => {
                let run_env = std::env::var("RUN_ENV").unwrap_or_else(|_| "".into());

                let file_path = format!(
                    ".env{}{}",
                    if run_env.is_empty() {
                        ""
                    } else {
                        "."
                    },
                    run_env
                );

                Self::load_env_file(&file_path)
                    .context("Failed to load .env file.")
                    .context(e.to_string())?;
                     

                envy::from_env::<Config>().context("Failed to load config.")
            }
        }
    }
}

impl Config {
    pub fn url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Config {
    fn load_env_file(file_path: &str) -> Result<(), std::io::Error> {
        let mut file = std::fs::File::open(file_path)?;

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        for line in content.lines() {
            let mut parts = line.splitn(2, '=');

            if let Some(key) = parts.next() {
                if let Some(value) = parts.next() {
                    std::env::set_var(key, value);
                }
            } else {
                eprintln!("Invalid line in .env file: {}", line);
            }
        }

        Ok(())
    }
}


