use tonic::transport::Server;
use monie_rpc::monie::authentication_server::AuthenticationServer;
use crate::rpc_server::AuthenticationService;

mod server;
mod domain;
mod data;
mod rpc_server;


#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    let authentication_service = AuthenticationService {};
    let authentication_server = AuthenticationServer::new(authentication_service);

    let addr = "127.0.0.1:8080".parse().unwrap();
    Server::builder().add_service(authentication_server).serve(addr).await
}