use std::env;
use http::Uri;
use aws_sdk_dynamodb::{Client, Endpoint, Config, Region};

pub fn is_dynamodb_host_set() -> bool {
  match env::var("DYNAMODB_HOST") {
    Ok(_s) => return true,
    _ => return false
  };
}

pub async fn create_dynamodb_client() -> Client {
  let is_host_set = is_dynamodb_host_set();

  if !is_host_set {
      let shared_config = aws_config::load_from_env().await;

      return Client::new(&shared_config);
  };

  let credentials_provider = aws_config::default_provider::credentials::default_provider().await;

  let uri = env::var("DYNAMODB_HOST").unwrap().parse::<Uri>().unwrap();

  let endpoint = Endpoint::immutable(uri);

  let conf = Config::builder().credentials_provider(credentials_provider).region(Region::new("us-east-1")).endpoint_resolver(endpoint).build();

  return Client::from_conf(conf);  
}