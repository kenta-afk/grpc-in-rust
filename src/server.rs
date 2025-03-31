use tonic::{Request, Response};
use tonic::transport::Server;
use crate::hello_world::greeter_server::{Greeter, GreeterServer};
use crate::hello_world::{HelloRequest, HelloResponse, GoodbyeRequest, GoodbyeResponse};
use crate::calculator::calculator_server::{Calculator, CalculatorServer};
use crate::calculator::{AddRequest, AddResponse};


pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub mod calculator {
    tonic::include_proto!("calculator");
}

#[derive(Default)]
pub struct MyGreeter {}
#[derive(Default)]
pub struct MyCalculator {}


#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, tonic::Status> {
        let reply = hello_world::HelloResponse {
            message: format!("Hello {} from server Side",request.into_inner().name)
        };

        Ok(Response::new(reply))
    }

    async fn say_goodbye(&self, request: Request<GoodbyeRequest>) -> Result<Response<GoodbyeResponse>, tonic::Status> {

        let req = request.into_inner();

        let reply = hello_world::GoodbyeResponse {
            message: format!("Goodbye {} from server Side", req.name),
            meeting: if req.reminder {
                "次回は、来週水曜日です。"
            } else {
                ""
            }.to_string(),
        };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl Calculator for MyCalculator {
    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddResponse>, tonic::Status> {
        let req = request.into_inner();
        let sum = req.a + req.b;
        let reply = calculator::AddResponse {
            result: sum

        };

        Ok(Response::new(reply))

    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "[::1]:50051".parse()?;
    let greeter_service = MyGreeter::default();
    let calculator_service = MyCalculator::default();
    
    println!("listening...");

    Server::builder()
        .add_service(GreeterServer::new(greeter_service))
        .add_service(CalculatorServer::new(calculator_service))
        .serve(address)
        .await?;
    Ok(())
}