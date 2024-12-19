mod config;
mod zerotier;
mod r#static;
use salvo::prelude::*;

use crate::r#static::index;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().push(Router::new().path("/<**>").get(index));

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
