#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    /// Returns a full connection string which contains:
    ///  - `postgres` specification
    ///  - username, password
    ///  - host, port
    ///  - database_name
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}/{database_name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            database_name = self.database_name,
        )
    }

    /// Returns a connection string without link to the database. It contains:
    ///  - `postgres` specification
    ///  - username, password
    ///  - host, port
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
        )
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialize our configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`
    // It will look for any top-level file with an extension
    //  that `config` knows how to parse: yaml, json, etc
    settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values it read into
    //  our Settings type
    settings.try_into()
}
