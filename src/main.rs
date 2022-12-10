use tokio::sync::mpsc;
use tonic::transport::Server;

use monie_rpc::monie::auth::authentication_server::AuthenticationServer;

use crate::authentication::AuthenticationService;

mod domain;
mod data;
mod authentication;

#[tokio::main]
async fn main() -> Result<(), tonic::transport::Error> {
    let addrs = ["127.0.0.1:5432", "127.0.0.1:50052"];
    let (tx, mut rx) = mpsc::unbounded_channel();

    for addr in addrs {
        let tx = tx.clone();

        let authentication_service = AuthenticationService {};
        let authentication_server = AuthenticationServer::new(authentication_service);

        let addr = addr.parse().unwrap();
        let serve = Server::builder()
            .concurrency_limit_per_connection(256)
            .add_service(authentication_server)
            .serve(addr);

        tokio::spawn(async move {
            if let Err(e) = serve.await {
                eprintln!("Error = {:?}", e);
            }

            tx.send(()).unwrap();
        });
    };

    rx.recv().await;
    Ok(())
}