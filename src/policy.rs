//    by John Smith
//
//  Created:
//    13 Feb 2024, 11:01:36
//  Last edited:
//    13 Feb 2024, 11:01:36
//  Auto updated?
//    No
//
//  Description:
//!   
//

use std::collections::HashMap;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::extract::cookie::PrivateCookieJar;
use brane_tsk::errors::StringError;

use crate::auth::AppState;


/***** HELPER MACROS *****/
/// Reports errors to stderr before sending back to user
macro_rules! err {
    ($code:expr, $err:expr) => {{
        use ::error_trace::ErrorTrace as _;

        // Trace the error
        let code = $code;
        let err = $err;
        ::log::error!("[{}] {}", code.to_string(), err.trace());

        // Return the plain version to the user
        (code, err.to_string())
    }};
}





//  Get all policy versions
pub async fn get_policies(State(state): State<AppState>, jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .get(format!("{}/v1/management/policies", state.checker_address))
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return err!(StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(), err);
        },
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => err!(StatusCode::INTERNAL_SERVER_ERROR, err),
    }
}

// Get specific version
pub async fn get_policy(State(state): State<AppState>, jar: PrivateCookieJar, Path(version): Path<i64>) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .get(format!("{}/v1/management/policies/{}", state.checker_address, version))
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return err!(StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(), err);
        },
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => err!(StatusCode::INTERNAL_SERVER_ERROR, err),
    }
}

// Get active version
pub async fn get_active_policy(State(state): State<AppState>, jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .get(format!("{}/v1/management/policies/active", state.checker_address))
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return err!(StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(), err);
        },
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => err!(StatusCode::INTERNAL_SERVER_ERROR, err),
    }
}

// Activate version
pub async fn post_activate_policy(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Json(version): Json<HashMap<String, i64>>,
) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .put(format!("{}/v1/management/policies/active", state.checker_address))
        .json(&version)
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return err!(StatusCode::from_u16(status).unwrap(), StringError(body));
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    (StatusCode::OK, body)
}

// Deactivate policy
pub async fn delete_deactivate_policy(State(state): State<AppState>, jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .delete(format!("{}/v1/management/policies/active", state.checker_address))
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return err!(StatusCode::from_u16(status).unwrap(), StringError(body));
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    (StatusCode::OK, body)
}

pub async fn post_add_policy(State(state): State<AppState>, jar: PrivateCookieJar, body: String) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return err!(StatusCode::UNAUTHORIZED, StringError("Unauthorized".into()));
        },
    };

    let client = reqwest::Client::new();
    let result = match client
        .post(format!("{}/v1/management/policies", state.checker_address))
        .body(body)
        .header("Authorization", format!("Bearer {}", policy_auth_token.value()))
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return err!(StatusCode::from_u16(status).unwrap(), StringError(body));
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return err!(StatusCode::INTERNAL_SERVER_ERROR, err);
        },
    };

    (StatusCode::OK, body)
}
