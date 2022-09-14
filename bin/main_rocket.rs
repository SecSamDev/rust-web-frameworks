use std::borrow::Cow;

use rust_web_wars::BasicUsername;
use rocket::{serde::{json::Json}, config::TlsConfig};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Json<BasicUsername<'static>> {
    Json(BasicUsername {
        username: Cow::Borrowed("Username1"),
        first_name: Cow::Borrowed("FirstName"),
        last_name: Cow::Borrowed("LastName"),
        password: Cow::Borrowed("SuperSecretPassword"),
        email: Cow::Borrowed("Email@email.com"),
        user_id: 0,
    })
}

#[post("/", format = "json", data = "<user>")]
fn echo(mut user : Json<BasicUsername>) -> Json<BasicUsername> {
    user.user_id = user.user_id + 1;
    user
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let mut config = rocket::Config::default();
    config.port = 10001;
    if cfg!(feature = "rustls") {
        config.tls = Some(TlsConfig::from_paths("./certs.crt", "./key.key"))
    }else{
        config.tls = None;
    }
    let _rocket = rocket::custom(config)
        .mount("/", routes![index, echo])
        .launch()
        .await?;

    Ok(())
}