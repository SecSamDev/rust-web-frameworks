use std::borrow::Cow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BasicUsername<'a> {
    pub username: Cow<'a, str>,
    pub first_name: Cow<'a, str>,
    pub last_name: Cow<'a, str>,
    pub password: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub user_id: u128
}