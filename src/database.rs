use anyhow::Result;
use neo4rs::{ConfigBuilder, Graph};
use std::sync::Arc;

/// Define a structure to manage the Neo4j connection.
#[derive(Clone, Debug)]
pub struct Neo4j {
    /// Neo4j connection as atomic reference.
    pub connection: Arc<Graph>,
}

/// Define a trait for the Neo4jManager with methods to interact with Neo4j.
#[async_trait::async_trait]
pub trait Neo4jManager {
    /// Execute page rank algorithm on graph. This may allow to detect highly connected transactionners within a second.
    async fn page_rank(&self) -> Result<()>;
    /// Execute community detection (Louvain method) algorithm on graph.
    async fn community_detection(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl Neo4jManager for Neo4j {
    /// Calculate page rank between all connected nodes in graph.
    async fn page_rank(&self) -> Result<()> {
        self.connection.run("MATCH p=(n:User)-[r]->(m:User) WHERE type(r) <> 'BLOCK' WITH project(p) as graph CALL pagerank_online.update(graph) YIELD node, rank SET node.rank = rank;").await?;

        Ok(())
    }

    /// Calculate communities between all connected nodes in graph.
    async fn community_detection(&self) -> Result<()> {
        self.connection.run("MATCH p=(n:User)-[r]->(m) WHERE type(r) <> 'BLOCK' AND type(r) <> 'VIEW' WITH project(p) as graph CALL community_detection_online.update(graph) YIELD node, community_id WITH node, community_id WHERE labels(node) = ['User'] SET node.community = community_id;").await?;

        Ok(())
    }
}

/// Initialize the connection for Neo4j using Bolt protocol.
pub async fn init(config: &crate::model::config::Config) -> Result<Arc<Graph>> {
    let neo_config = ConfigBuilder::default()
        .uri(config.database.bolt.hosts[0])
        .user(config.database.bolt.username)
        .password(config.database.bolt.password)
        .max_connections(config.database.bolt.pool_size)
        .build()?;

    Ok(Arc::new(Graph::connect(neo_config).await?))
}
