use axum::{
    body::Body,
    http::{Request, StatusCode, Uri},
    response::Response,
    routing::get_service,
    Router,
};
use std::{convert::Infallible, net::SocketAddr, path::PathBuf};
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(fallback_service())
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn fallback_service() -> axum::routing::BoxRoute<Body> {
    axum::routing::get_service(get_static_service).handle_error(handle_error).boxed()
}

async fn get_static_service(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let host = req
        .headers()
        .get("host")
        .and_then(|h| h.to_str().ok())
        .unwrap_or_default();

    let static_folder = match host {
        "example1.com" => "static1",
        "example2.com" => "static2",
        _ => "static_default",
    };

    let uri = req.uri().clone();
    let path = PathBuf::from(format!("./{}/{}", static_folder, uri.path().trim_start_matches('/')));

    match ServeDir::new(path).oneshot(req).await {
        Ok(res) => Ok(res),
        Err(_) => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("File not found"))
            .unwrap()),
    }
}

async fn handle_error(error: std::io::Error) -> impl axum::response::IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, format!("Unhandled internal error: {}", error))
}
