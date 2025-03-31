use tonic::{Request, Response};
use tonic::transport::Server;
use crate::hello_world::greeter_server::Greeter;
use crate::hello_world::HelloRequest;
use crate::hello_world::HelloResponse;
use crate::hello_world::greeter_server::GreeterServer;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Default)]
pub struct MyGreeter {}


#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, tonic::Status> {
        let reply = hello_world::HelloResponse {
            message: format!("Hello {} from server Side",request.into_inner().name)
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let greeter_service = MyGreeter::default();
    
    println!("listening...");

    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .serve(address)
        .await?;
    Ok(())
}