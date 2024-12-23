mod config;
mod zerotier;

mod statics;
use statics::index;

mod api;
use api::*;

use salvo::prelude::*;

use clap::Parser;

lazy_static::lazy_static! {
    static ref CONFIG: config::AppConfig = config::AppConfig::init(Args::parse().config);
    static ref ZEROTIER: zerotier::ZeroTier = zerotier::ZeroTier::new(&CONFIG);
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "config.json")]
    config: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new()
        .push(Router::with_path("api").push(Router::with_path("login").post(login)))
        .push(Router::with_path("ztapi/<**>").goal(forward_to_zt))
        .push(Router::with_path("<**>").get(index));

    let acceptor = TcpListener::new(CONFIG.listen.clone()).bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn forward_to_zt(req: &mut Request, res: &mut Response) {
    let path = req.uri().path().replace("/ztapi/", "");

    let response = ZEROTIER.forward(&path, req.method().clone()).await;

    match response {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap();

            res.status_code(status);
            res.render(Json(body));
        }
        Err(e) => {
            res.render(StatusError::bad_request().brief(e.to_string()));
        }
    }
}
