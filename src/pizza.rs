use tonic::{metadata::MetadataValue, Request, Response, Status};
use aws_sdk_dynamodb::model::AttributeValue;
use std::env;

mod aws;

pub mod grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("pizza"); 
}

pub use grpc::pizza_requests_server::{PizzaRequests, PizzaRequestsServer};

use grpc::{Pizza, GetPizzaRequest};

#[derive(Debug, Default)]
pub struct PizzaService {}

pub fn check_auth(req: Request<()>) -> Result<Request<()>, Status> {
    let token = MetadataValue::from_str("Bearer some-secret-token").unwrap();

    match req.metadata().get("authorization") {
        Some(t) if token == t => Ok(req),
        _ => Err(Status::unauthenticated("No valid auth token")),
    }
}

#[tonic::async_trait]
impl PizzaRequests for PizzaService {
    async fn store_pizza(&self, request: Request<Pizza>) -> Result<Response<Pizza>, Status> {  
        println!("store_pizza request: {:?}", request);

        let pizza_table = env::var("PIZZA_TABLE").unwrap();

        let client = aws::create_dynamodb_client().await;

        let pizza: grpc::Pizza = request.into_inner();

        let request = client.put_item().table_name(&pizza_table).item("id", AttributeValue::S(pizza.id.clone())).item("name", AttributeValue::S(pizza.name.clone()));

        match request.send().await {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::internal(String::from("testing")));
            }
        };

        return Ok(Response::new(pizza));
    }

    async fn get_pizza(&self, request: Request<GetPizzaRequest>) -> Result<Response<Pizza>, Status> {
        println!("get_pizza request: {:?}", request);

        let pizza_table = env::var("PIZZA_TABLE").unwrap();

        let client = aws::create_dynamodb_client().await;

        let params: GetPizzaRequest = request.into_inner();

        let request = client.get_item().table_name(&pizza_table).key("id", AttributeValue::S(params.id.clone()));

        let response = match request.send().await {
            Ok(res) => res,
            Err(err) => {
                println!("{:?}", err);
                return Err(Status::internal(String::from("testing")));
            }
        };

        let pizza = match response.item {
            Some(item) => item,
            None => {
                return Err(Status::internal(format!("pizza with id '{}' does not exist", params.id)));
            }
        };

        let missing_attribute_value_error = Status::internal(format!("pizza with id '{}' missing attribute value", params.id));

        let id = match pizza.get("id") {
            Some(val) => val.as_s().unwrap().to_string(),
            None => return Err(missing_attribute_value_error)
        };

        let name = match pizza.get("name") {
            Some(val) => val.as_s().unwrap().to_string(),
            None => return Err(missing_attribute_value_error)
        };

        let reply = grpc::Pizza {
            id: id,
            name: name
        };

        return Ok(Response::new(reply));
    }
}
