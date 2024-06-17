use axum::{
    extract::TypedHeader,
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Router,
};
use hyper::header::HOST;
use std::net::SocketAddr;
use tokio::main;

#[tokio::main]
async fn main() {
    // Build the application with a route
    let app = Router::new().route("/", get(domain_handler));

    // Specify the address and port to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Run the server
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler function that extracts and prints the domain name
async fn domain_handler(TypedHeader(headers): TypedHeader<HeaderMap>) -> impl IntoResponse {
    if let Some(host) = headers.get(HOST) {
        if let Ok(host_str) = host.to_str() {
            println!("Accessed from domain: {}", host_str);
            return format!("Accessed from domain: {}", host_str);
        }
    }

    "Could not determine the domain name".to_string()
}

