use axum::{
    body::Body,
    extract::{ContentLengthLimit, Multipart, Path, TypedHeader},
    handler::Handler,
    headers::{HeaderMap, Host},
    http::{Request, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post, Router},
    AddExtensionLayer,
};
use askama::Template;
use hyper::Server;
use moka::sync::Cache;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{
    collections::HashMap,
    fs::File,
    io::BufReader,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::fs;
use tokio_rustls::{TlsAcceptor, TlsConnector};
use uuid::Uuid;
use async_trait::async_trait;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    domain: &'a str,
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    path: &'a str,
}

struct DomainConfig {
    static_folder: String,
    upload_folder: String,
    tls_cert: Vec<Certificate>,
    tls_key: PrivateKey,
}

struct AppState {
    domains: Mutex<HashMap<String, DomainConfig>>,
    cookie_jar: Arc<dyn CookieJar>,
}

#[async_trait]
trait CookieJar {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: String);
    async fn remove(&self, key: &str);
}

struct MokaCookieJar {
    cache: Cache<String, String>,
}

#[async_trait]
impl CookieJar for MokaCookieJar {
    async fn get(&self, key: &str) -> Option<String> {
        self.cache.get(key).cloned()
    }

    async fn set(&self, key: &str, value: String) {
        self.cache.insert(key.to_string(), value);
    }

    async fn remove(&self, key: &str) {
        self.cache.invalidate(key);
    }
}

async fn serve_static(
    TypedHeader(headers): TypedHeader<HeaderMap>,
    Path(path): Path<String>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    if let Some(host) = headers.get(Host::NAME) {
        if let Ok(domain) = host.to_str() {
            let domains = state.domains.lock().unwrap();
            if let Some(config) = domains.get(domain) {
                let file_path = format!("{}/{}", config.static_folder, path);
                if let Ok(contents) = fs::read(file_path).await {
                    return (StatusCode::OK, contents.into_response());
                } else {
                    let not_found = NotFoundTemplate { path: &path };
                    return (StatusCode::NOT_FOUND, Html(not_found.render().unwrap()).into_response());
                }
            }
        }
    }
    (StatusCode::NOT_FOUND, "Domain not found".into_response())
}

async fn handle_upload(
    TypedHeader(headers): TypedHeader<HeaderMap>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<Multipart, { 10 * 1024 * 1024 }>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    if let Some(host) = headers.get(Host::NAME) {
        if let Ok(domain) = host.to_str() {
            let domains = state.domains.lock().unwrap();
            if let Some(config) = domains.get(domain) {
                while let Some(field) = multipart.next_field().await.unwrap() {
                    let file_name = field.file_name().unwrap().to_string();
                    let file_path = format!("{}/{}", config.upload_folder, file_name);
                    let data = field.bytes().await.unwrap();
                    fs::write(file_path, data).await.unwrap();
                }
                return StatusCode::OK.into_response();
            }
        }
    }
    StatusCode::NOT_FOUND.into_response()
}

async fn login_handler(
    Query(params): Query<HashMap<String, String>>,
    state: Arc<AppState>,
) -> impl IntoResponse {
    let username = params.get("username").cloned();
    let password = params.get("password").cloned();

    if let (Some(username), Some(password)) = (username, password) {
        if validate_user(&username, &password).await {
            let session_id = Uuid::new_v4().to_string();
            state.cookie_jar.set(&session_id, username).await;
            (StatusCode::OK, Html("Login successful".to_string()))
        } else {
            (StatusCode::UNAUTHORIZED, Html("Invalid credentials".to_string()))
        }
    } else {
        (StatusCode::BAD_REQUEST, Html("Missing username or password".to_string()))
    }
}

async fn validate_user(username: &str, password: &str) -> bool {
    // Replace this with your user validation logic
    username == "user" && password == "password"
}

async fn private_data_handler(state: Arc<AppState>, req: Request<Body>) -> impl IntoResponse {
    if let Some(cookie_header) = req.headers().get("cookie") {
        let cookie = cookie_header.to_str().unwrap_or("");
        if let Some(session_id) = cookie.split('=').nth(1) {
            if let Some(username) = state.cookie_jar.get(session_id).await {
                return (StatusCode::OK, Html(format!("Private data for user: {}", username)));
            }
        }
    }
    (StatusCode::UNAUTHORIZED, Html("Access denied".to_string()))
}

async fn public_data_handler() -> impl IntoResponse {
    Html("Public data".to_string())
}

async fn index_handler(TypedHeader(headers): TypedHeader<HeaderMap>, state: Arc<AppState>) -> impl IntoResponse {
    if let Some(host) = headers.get(Host::NAME) {
        if let Ok(domain) = host.to_str() {
            let index = IndexTemplate { domain };
            return Html(index.render().unwrap()).into_response();
        }
    }
    (StatusCode::NOT_FOUND, "Domain not found".into_response())
}

fn load_certs(path: &str) -> Vec<Certificate> {
    certs(&mut BufReader::new(File::open(path).unwrap()))
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect()
}

fn load_private_key(path: &str) -> PrivateKey {
    let keys = pkcs8_private_keys(&mut BufReader::new(File::open(path).unwrap())).unwrap();
    PrivateKey(keys.into_iter().next().unwrap())
}

#[tokio::main]
async fn main() {
    let domains = Mutex::new(HashMap::new());
    let example_domain_config = DomainConfig {
        static_folder: "static".to_string(),
        upload_folder: "uploads".to_string(),
        tls_cert: load_certs("path/to/cert1.pem"),
        tls_key: load_private_key("path/to/key1.pem"),
    };
    let another_example_domain_config = DomainConfig {
        static_folder: "static".to_string(),
        upload_folder: "uploads".to_string(),
        tls_cert: load_certs("path/to/cert2.pem"),
        tls_key: load_private_key("path/to/key2.pem"),
    };
    domains.lock().unwrap().insert("example.com".to_string(), example_domain_config);
    domains.lock().unwrap().insert("another-example.com".to_string(), another_example_domain_config);

    let app_state = Arc::new(AppState {
        domains,
        cookie_jar: Arc::new(MokaCookieJar {
            cache: Cache::new(100),
        }),
    });

    let app = Router::new()
        .route("/:domain/*path", get(serve_static))
        .route("/:domain/upload", post(handle_upload))
        .route("/login", post(login_handler))
        .route("/data/private", get(private_data_handler))
        .route("/data/public", get(public_data_handler))
        .route("/:domain", get(index_handler))
        .layer(AddExtensionLayer::new(app_state));

    let tls_config = {
        let resolver = ResolvesServerCertUsingSni::new();

        {
            let domains = app_state.domains.lock().unwrap();
            for (domain, config) in domains.iter() {
                let key = CertifiedKey::new(config.tls_cert.clone(), any_supported_type(&config.tls_key).unwrap());
                resolver.add(domain.as_str(), key).unwrap();
            }
        }

        ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_cert_resolver(Arc::new(resolver))
    };

    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let acceptor = tls_acceptor.clone();
        let app = app.clone();

        tokio::spawn(async move {
            let stream = acceptor.accept(stream).await.unwrap();
            hyper::server::conn::Http::new().serve_connection(stream, app).await.unwrap();
        });
    }
}
