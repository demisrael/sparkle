use acme_client::Client;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() {
    // Define the directory URL for Let's Encrypt
    let directory_url = "https://acme-v02.api.letsencrypt.org/directory";

    // Create a new ACME client
    let client = Client::new(directory_url).unwrap();

    // Generate a new account and key
    let account = client.new_account("mailto:your-email@example.com", true).unwrap();

    // Define the domain names you want to secure
    let domains = vec!["yourdomain.com", "www.yourdomain.com"];

    // Generate a new order
    let mut order = client.new_order(domains.clone()).unwrap();

    // Get the list of authorizations that need to be completed
    for auth in order.authorizations().unwrap() {
        let http_challenge = auth.http_challenge().unwrap();

        // Print the challenge token and response
        println!("HTTP Challenge URL: {}", http_challenge.url());
        println!("HTTP Challenge Token: {}", http_challenge.token());
        println!("HTTP Challenge Key Authorization: {}", http_challenge.key_authorization().unwrap());

        // Serve the HTTP challenge response on your web server (code not included here)
        // Ensure the challenge is accessible at http://yourdomain.com/.well-known/acme-challenge/{token}
        // Example: http://yourdomain.com/.well-known/acme-challenge/<token> should serve the key_authorization

        // Validate the challenge
        http_challenge.validate().unwrap();
    }

    // Finalize the order
    let csr = acme_client::openssl::generate_csr(&domains).unwrap();
    order.finalize(csr).unwrap();

    // Download the certificate
    let cert = order.download_cert().unwrap();

    // Save the certificate and private key to files
    let mut cert_file = File::create("fullchain.pem").unwrap();
    cert_file.write_all(cert.certificate().as_bytes()).unwrap();

    let mut key_file = File::create("private_key.pem").unwrap();
    key_file.write_all(cert.private_key().as_bytes()).unwrap();

    println!("Certificate and private key have been saved.");
}
