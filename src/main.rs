use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(hello_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn hello_handler() -> impl IntoResponse {
    Html("hello rusty")
}
