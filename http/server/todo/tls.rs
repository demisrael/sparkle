use axum::{Router, routing::get};
use hyper::Server;
use std::{sync::Arc, net::SocketAddr};
use tokio::net::TcpListener;
use tokio_rustls::{
    rustls::{
        ServerConfig, 
        Certificate, 
        PrivateKey, 
        ServerName,
        ResolvesServerCertUsingSni,
        sign::{any_supported_type, CertifiedKey},
    },
    TlsAcceptor,
};
use rustls_pemfile::{read_one, Item};
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

async fn handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // Define your Axum app
    let app = Router::new().route("/", get(handler));

    // Load certificates and private keys
    let cert1 = load_certs("path/to/cert1.pem");
    let key1 = load_private_key("path/to/key1.pem");
    let cert2 = load_certs("path/to/cert2.pem");
    let key2 = load_private_key("path/to/key2.pem");

    // Create the certified keys
    let key1 = CertifiedKey::new(cert1, any_supported_type(&key1).unwrap());
    let key2 = CertifiedKey::new(cert2, any_supported_type(&key2).unwrap());

    // Create a resolver
    let mut resolver = ResolvesServerCertUsingSni::new();
    resolver.add("example.com", key1).unwrap();
    resolver.add("another-example.com", key2).unwrap();

    // Build the rustls ServerConfig
    let tls_config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_cert_resolver(Arc::new(resolver));

    // Wrap the Axum app in a TlsAcceptor
    let tls_acceptor = TlsAcceptor::from(Arc::new(tls_config));

    // Bind to an address and serve the app
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(&addr).await.unwrap();

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

fn load_certs(path: &str) -> Vec<Certificate> {
    let certfile = File::open(path).unwrap();
    let mut reader = BufReader::new(certfile);
    let certs = rustls_pemfile::certs(&mut reader)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();
    certs
}

fn load_private_key(path: &str) -> PrivateKey {
    let keyfile = File::open(path).unwrap();
    let mut reader = BufReader::new(keyfile);
    loop {
        match read_one(&mut reader).unwrap() {
            Some(Item::RSAKey(key)) | Some(Item::PKCS8Key(key)) | Some(Item::ECKey(key)) => return PrivateKey(key),
            None => break,
            _ => (),
        }
    }
    panic!("no keys found in {:?}", path);
}
