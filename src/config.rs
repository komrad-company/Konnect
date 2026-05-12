use serde::Deserialize;

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
    pub(crate) fn connection_url(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}?options=-csearch_path%3D{}",
            self.user, self.password, self.host, self.port, self.database, self.search_path
        )
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
    fn connection_url_contains_search_path() {
        let url = config().connection_url();
        assert!(url.contains("search_path%3Dkorelator"), "url: {url}");
    }

    #[test]
    fn connection_url_format_is_valid() {
        let url = config().connection_url();
        assert_eq!(
            url,
            "postgresql://korelator:secret@localhost:5432/komrad?options=-csearch_path%3Dkorelator"
        );
    }
}
