use serde::Deserialize;
use sqlx::postgres::PgConnectOptions;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub schema: String,
    pub search_path: String,
}

impl DatabaseConfig {
    pub(crate) fn connect_options(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .database(&self.database)
            .username(&self.user)
            .password(&self.password)
            .options([("search_path", self.search_path.as_str())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> DatabaseConfig {
        DatabaseConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "komrad".to_string(),
            user: "korelator".to_string(),
            password: "secret".to_string(),
            schema: "korelator".to_string(),
            search_path: "korelator".to_string(),
        }
    }

    #[test]
    fn database_config_deserializes_correctly() {
        let config: DatabaseConfig = serde_json::from_str(
            r#"{"host":"localhost","port":5432,"database":"komrad","user":"korelator","password":"secret","schema":"korelator","search_path":"korelator"}"#,
        )
        .unwrap();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.search_path, "korelator");
    }

    #[test]
    fn special_characters_in_password_do_not_panic() {
        let mut cfg = config();
        cfg.password = "p@ss/word?#weird%".to_string();
        let _ = cfg.connect_options();
    }
}
