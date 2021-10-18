use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub name: Option<String>,
}

impl DatabaseSettings {
    fn from_env() -> Result<Self> {
        Ok(envy::prefixed("DB_").from_env::<DatabaseSettings>()?)
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.name.as_ref().unwrap(),
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port,
        )
    }
}

impl Settings {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            application_port: dotenv::var("APPLICATION_PORT")?
                .parse::<u16>()?,
            database: DatabaseSettings::from_env()?,
        })
    }
}
