mod cli;
mod log_util;
mod web;

use std::net::SocketAddr;

use clap::Parser;
use cli::Cli;
use log::info;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let parse = Cli::parse();
    let port: u16 = 3000;
    println!("conf-path: {:?}", parse.conf_path());
    println!("templates-path: {:?}", parse.templates_path());
    let router = web::build_app().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("app built and running on port {p}", p = port);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
