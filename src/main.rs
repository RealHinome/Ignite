use tonic::{transport::Server, Request, Response, Status};

use ignite::ignite_server::{Torre, TorreServer};
use ignite::{VerifyRequest, VerifyResponse};

mod helpers;
mod model;
mod database;
pub mod ignite {
    //tonic::include_proto!("ignite");
}

pub struct Ignite {
    graph: database::Neo4j
}

#[tonic::async_trait]
impl Torre for Ignite {
    async fn torre_predict(
        &self,
        request: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        Ok(Response::new(VerifyResponse {
            fraud: false,
            accuracy: 0
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = helpers::config::read();

    // Start database
    let neo4j = database::Neo4j { connection: database::init(&config) };

    let addr = format!("0.0.0.0:{}", config.port).parse().unwrap();

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(TorreServer::new(Ignite { graph: neo4j }))
        .serve(addr)
        .await?;

    Ok(())
}