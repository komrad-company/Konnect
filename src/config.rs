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
