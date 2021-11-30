use std::env;
use tonic::{transport::Server};

mod pizza;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::var("PORT").unwrap_or(String::from("50051"));

    let bind_address = env::var("ADDRESS").unwrap_or(String::from("0.0.0.0"));

    let url = format!("{}:{}", bind_address, port);

    let addr = url.parse()?;

    let server = pizza::PizzaService::default();

    // let service = pizza::PizzaRequestsServer::with_interceptor(server, pizza::check_auth);
    let service = pizza::PizzaRequestsServer::new(server);

    println!("gRPC server listening on {}", url);

    Server::builder()
        .accept_http1(true)
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}