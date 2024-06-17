use axum::{
    body::{Body, StreamBody},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use tokio::fs;
use tokio_util::io::ReaderStream;

async fn get_file() -> Result<Response, StatusCode> {
    let path = Path::new("path/to/your/file.pdf");
    let file = fs::File::open(path).await.map_err(|_| StatusCode::NOT_FOUND)?;

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=\"file.pdf\"")
        .body(body)
        .unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/download", get(get_file));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
