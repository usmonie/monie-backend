use std::net::SocketAddr;
use tokio::net::TcpListener;

use tonic::transport::Server;

use monie_rpc::monie::auth::authentication_server::AuthenticationServer;

use crate::authentication::AuthenticationService;

mod domain;
mod data;
mod authentication;

fn main(){
    let mut handlers = Vec::new();
    for i in 0..num_cpus::get() {
        let h = std::thread::spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(serve());
        });
        handlers.push(h);
    }

    for h in handlers {
        h.join().unwrap();
    }
}

async fn serve() {
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let sock = socket2::Socket::new(
        match addr {
            SocketAddr::V4(_) => socket2::Domain::IPV4,
            SocketAddr::V6(_) => socket2::Domain::IPV6,
        },
        socket2::Type::STREAM,
        None,
    ).unwrap();

    let authentication_service = AuthenticationService {};
    let authentication_server = AuthenticationServer::new(authentication_service);

    sock.set_reuse_address(true).unwrap();
    sock.set_reuse_port(true).unwrap();
    sock.set_nonblocking(true).unwrap();
    sock.bind(&addr.into()).unwrap();
    sock.listen(8192).unwrap();

    let incoming =
        tokio_stream::wrappers::TcpListenerStream::new(TcpListener::from_std(sock.into()).unwrap());

    Server::builder()
        .concurrency_limit_per_connection(256)
        .add_service(authentication_server)
        .serve_with_incoming(incoming)
        .await
        .unwrap();
}