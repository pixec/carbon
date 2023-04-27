use std::net::SocketAddr;
use std::str::FromStr;
use carbon::system_server::{System, SystemServer};
use carbon::{InformationRequest, InformationReply};
use tonic::transport::Server;
use tonic::{Request, Result, Response, Status};

pub mod carbon {
    tonic::include_proto!("carbon");
}

#[derive(Default)]
struct SystemService {}

#[tonic::async_trait]
impl System for SystemService {
    async fn get_information(&self, _request: Request<InformationRequest>) -> Result<Response<InformationReply>, Status> {
        println!("Got a request!");
        
        let reply = InformationReply {
            uptime: 0,
        };
        
        Ok(Response::new(reply))
    }
}

#[tokio::main]

async fn main() {
    Server::builder()
        .add_service(SystemServer::new(SystemService::default()))
        .serve(SocketAddr::from_str("0.0.0.0:3000").unwrap())
        .await
        .unwrap();
}
