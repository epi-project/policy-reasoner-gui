use std::fs::{read_to_string, File};
use std::io::Write;

use axum::extract::FromRef;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, Key};
use axum_extra::extract::PrivateCookieJar;
use base64::engine::general_purpose;
use base64::Engine as _;
use serde::{Deserialize, Serialize};
use specifications::address::Address;

pub fn get_key(p: &str) -> Key {
    match read_to_string::<&str>(p.into()) {
        Ok(contents) => {
            let key_bytes = general_purpose::STANDARD
                .decode(&contents)
                .expect("Invalid key bytes");

            Key::from(&key_bytes)
        }
        Err(_) => {
            let key = Key::generate();

            let b = key.master();
            let b64key = general_purpose::STANDARD.encode(&b);

            let mut output = File::create(p).expect("Could not create file to store key");
            write!(output, "{}", b64key).expect("Could not write key to file");
            // output.write(b64key.into());
            output.flush().expect("Could not flush key to file");
            key
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AuthDataType {
    Policy,
    Deliberation,
}

#[derive(Serialize)]
pub struct AuthDataViewModel {
    pub policy: String,
    pub deliberation: String,
}

impl AuthDataViewModel {
    fn new() -> Self {
        Self {
            policy: "".into(),
            deliberation: "".into(),
        }
    }
}

#[derive(Deserialize)]
pub struct AuthDataPostModel {
    pub t: AuthDataType,
    pub token: String,
}

// our application state
#[derive(Clone)]
pub struct AppState {
    /// The checker address to connect to.
    pub checker_address: Address,
    // that holds the key used to sign cookies
    pub key: Key,
}

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

// #[debug_handler]
pub async fn post_authenticate(
    jar: PrivateCookieJar,
    Json(auth_data): Json<AuthDataPostModel>,
) -> (PrivateCookieJar, StatusCode) {
    match auth_data.t {
        AuthDataType::Policy => {
            let mut cookie = Cookie::new("reasoner_policy_auth", auth_data.token);
            cookie.set_secure(true);
            cookie.set_http_only(true);
            // cookie.set_max_age(Some(Duration::new(60 * 10, 0)));
            (jar.add(cookie), StatusCode::OK)
        }
        AuthDataType::Deliberation => {
            let mut cookie = Cookie::new("reasoner_deliberation_auth", auth_data.token);
            cookie.set_secure(true);
            cookie.set_http_only(true);
            // cookie.set_max_age(Some(Duration::new(60 * 10, 0)));
            (jar.add(cookie), StatusCode::OK)
        }
    }
}

pub async fn get_authenticate(jar: PrivateCookieJar) -> (StatusCode, Json<AuthDataViewModel>) {
    let mut auth_data = AuthDataViewModel::new();

    if let Some(data) = jar.get("reasoner_policy_auth") {
        auth_data.policy = data.value().into();
    }
    if let Some(data) = jar.get("reasoner_deliberation_auth") {
        auth_data.deliberation = data.value().into();
    }

    (StatusCode::OK, Json(auth_data))
}

pub async fn logout(jar: PrivateCookieJar) -> (PrivateCookieJar, StatusCode) {
    let jar = jar
        .remove("reasoner_policy_auth")
        .remove("reasoner_deliberation_auth");

    (jar, StatusCode::OK)
}
