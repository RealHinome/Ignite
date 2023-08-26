use anyhow::Result;
use neo4rs::{Graph, ConfigBuilder};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Neo4j {
    pub connection: Arc<Graph>,
}

pub trait Neo4jManager {
}

impl Neo4jManager for Neo4j {
}

/// Initialize the connection for Neo4j using Bolt protocol.
pub async fn init(
    config: &crate::model::config::Config,
) -> Result<Arc<Graph>> {
    let neo_config = ConfigBuilder {
        uri: Some(config.database.bolt.hosts[0]),
        user: config.database.bolt.username,
        password: config.database.bolt.password,
        db: "neo4j".into(),
        fetch_size: 200,
        max_connections: config.database.bolt.pool_size,
    };

    Ok(
        Arc::new(
            Graph::connect(neo_config)
            .await?
        )
    )
}