use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use serde::Deserialize;

#[tokio::main]
async fn main() {
    let all_routes = Router::new().merge(routes_hello());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(all_routes.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello_handler(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("default");
    Html(format!("hello rusty {name}"))
}
