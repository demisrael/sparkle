// [dependencies]
// axum = "0.5"
// tokio = { version = "1", features = ["full"] }
// hyper = { version = "0.14", features = ["full"] }
// tower = "0.4"
// multipart = "0.3"
// tokio-util = "0.6"


use axum::{
    extract::{ContentLengthLimit, Multipart},
    handler::post,
    http::StatusCode,
    response::Html,
    Router,
};
use std::net::SocketAddr;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/upload", post(upload));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127.0.0.1, 3000]));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// handler for file upload
async fn upload(
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { 10 * 1024 * 1024 * 1024 }>, // Limit to 10GB
) -> Result<Html<&'static str>, StatusCode> {
    // Iterate over the fields
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();

        let file_path = format!("./uploads/{}", file_name);
        let mut file = File::create(file_path).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut field_stream = field.compat();
        let mut buffer = [0u8; 8 * 1024]; // 8 KB buffer

        while let Ok(bytes_read) = field_stream.read(&mut buffer).await {
            if bytes_read == 0 {
                break;
            }
            file.write_all(&buffer[..bytes_read]).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

        println!("Uploaded file: {} (field name: {})", file_name, name);
    }

    Ok(Html("<h1>Upload complete</h1>"))
}
