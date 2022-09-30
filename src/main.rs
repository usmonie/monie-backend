mod feed;
mod domain;
mod data;
mod error_handlers;

use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use listenfd::ListenFd;
use tokio::time::Instant;
use dotenv::dotenv;
use crate::data::db::db;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[tokio::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("{}", Instant::now().elapsed().as_nanos());
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    println!("Hello after listenfd = ");
    let mut server = HttpServer::new(|| App::new()
        .service(feed::feed)
        .service(auth));
    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");

            server.bind(format!("{}:{}", host, port))?
        }
    };
    println!("Hello after server = ");

    server.run().await

}


#[get("/auth/register")]
async fn auth() -> impl Responder {
    "Hello ".to_string()
}

