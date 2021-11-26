use std::env;
use tonic::{transport::Server};

mod pizza;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or(String::from("50051"));

    let url = format!("[::1]:{}", port);

    let addr = url.parse()?;

    let server = pizza::PizzaService::default();

    let service = pizza::PizzaRequestsServer::with_interceptor(server, pizza::check_auth);

    Server::builder()
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}