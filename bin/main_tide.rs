use rust_web_wars::BasicUsername;
use tide::{Request, Body};
use tide_rustls::TlsListener;
use std::{borrow::Cow};

#[derive(Clone)]
pub struct BasicState {}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let state = BasicState{};
    let mut app = tide::with_state(state);

    app.at("/")
    .post(|mut req: Request<BasicState>| async move {
        let mut username : BasicUsername = req.body_json().await?;
        username.user_id = username.user_id + 1;
        Body::from_json(&username)
    })
    .get(|mut _req: Request<BasicState>| async move {
        let username = BasicUsername {
            username: Cow::Borrowed("Username1"),
            first_name: Cow::Borrowed("FirstName"),
            last_name: Cow::Borrowed("LastName"),
            password: Cow::Borrowed("SuperSecretPassword"),
            email: Cow::Borrowed("Email@email.com"),
            user_id: 0,
        };
        Body::from_json(&username)
    });
    if cfg!(feature = "rustls") {
        app.listen(
            TlsListener::build()
                .addrs("0.0.0.0:10001")
                .cert("./certs.crt")
                .key("./key.key"),
            )
            .await?;
    }else{
        app.listen("0.0.0.0:10001").await?;
    }
    Ok(())
}
