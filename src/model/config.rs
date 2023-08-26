use serde::Deserialize;

/// Connection represents database informations to connect
#[derive(Deserialize, Debug, Clone)]
pub struct Connection {
    pub username: Option<String>,
    pub password: Option<String>,
    pub hosts: Vec<String>,
    pub pool_size: usize,
}

/// Database represents the databases connection datas
#[derive(Deserialize, Debug, Clone)]
pub struct Database {
    pub bolt: Connection,
}

/// Config represents how config.yaml should be
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub database: Database,
}