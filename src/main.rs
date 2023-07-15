use salvo::prelude::*;

mod handlers;
mod routers;
mod schema;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let middleware = Router::new().hoop(Logger::new());
    let api = middleware.push(routers::api());

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(api).await;
}
