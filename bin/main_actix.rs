use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use rustls::{ServerConfig, Certificate, PrivateKey};

use std::borrow::Cow;
use std::io::{BufReader};
use std::fs::File;
use rust_web_wars::BasicUsername;

#[post("/")]
async fn echo(mut username : web::Json<BasicUsername<'_>>) -> impl Responder {
    username.user_id = username.user_id + 1;
    HttpResponse::Ok().json(username)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(BasicUsername {
        username: Cow::Borrowed("Username1"),
        first_name: Cow::Borrowed("FirstName"),
        last_name: Cow::Borrowed("LastName"),
        password: Cow::Borrowed("SuperSecretPassword"),
        email: Cow::Borrowed("Email@email.com"),
        user_id: 0,
    })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = ("0.0.0.0", 10001);
    let mut server = HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
    });
    //.bind_rustls(addrs, config)
    server = if cfg!(feature = "rustls") {
        let certs = File::open("certs.crt").expect("Must exist the certs.crt file");
        let mut certs = BufReader::new(certs);
        let certs = rustls_pemfile::certs(&mut certs)?.into_iter().map(|v| Certificate(v)).collect();

        let private_key = File::open("key.key").expect("Must exist the key.key file");
        let mut private_key = BufReader::new(private_key);
        let mut private_key : Vec<PrivateKey> = rustls_pemfile::rsa_private_keys(&mut private_key)?.into_iter().take(1).map(|v| PrivateKey(v)).collect();
        let private_key = private_key.pop().unwrap();
        
        let config = ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth()
            .with_single_cert(certs, private_key).expect("Must generate TLS server config");
        server.bind_rustls(addr, config)?
    }else{
        server.bind(addr)?
    };
    
    server.run().await
}
