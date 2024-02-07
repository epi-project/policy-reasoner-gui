use axum::{extract::Path, http::StatusCode, Json};
use axum_extra::extract::cookie::PrivateCookieJar;
use std::collections::HashMap;

// Get all policy versions
pub async fn get_policies(jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .get("http://localhost:3030/v1/management/policies/versions")
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return (
                StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(),
                err.to_string(),
            );
        }
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err)),
    }
}

// Get specific version
pub async fn get_policy(jar: PrivateCookieJar, Path(version): Path<i64>) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .get(format!(
            "http://localhost:3030/v1/management/policies/{}",
            version
        ))
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return (
                StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(),
                err.to_string(),
            );
        }
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err)),
    }
}

// Get active version
pub async fn get_active_policy(jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .get("http://localhost:3030/v1/management/policies/active")
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let body = match result.error_for_status() {
        Ok(result) => result.text().await,
        Err(err) => {
            return (
                StatusCode::from_u16(err.status().unwrap().as_u16()).unwrap(),
                err.to_string(),
            );
        }
    };

    match body {
        Ok(data) => (StatusCode::OK, data),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err)),
    }
}

// Activate version
pub async fn post_activate_policy(
    jar: PrivateCookieJar,
    Json(version): Json<HashMap<String, i64>>,
) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .put("http://localhost:3030/v1/management/policies/active")
        .json(&version)
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return (StatusCode::from_u16(status).unwrap(), body);
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    (StatusCode::OK, body)
}

// Deactivate policy
pub async fn delete_deactivate_policy(jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .delete("http://localhost:3030/v1/management/policies/active")
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return (StatusCode::from_u16(status).unwrap(), body);
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    (StatusCode::OK, body)
}

pub async fn post_add_policy(jar: PrivateCookieJar, body: String) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .post("http://localhost:3030/v1/management/policies")
        .body(body)
        .header(
            "Authorization",
            format!("Bearer {}", policy_auth_token.value()),
        )
        .header("Content-Type", "application/json")
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    let status = result.status().as_u16();

    if status > 399 {
        let body = match result.text().await {
            Ok(text) => text,
            Err(_) => "".into(),
        };

        return (StatusCode::from_u16(status).unwrap(), body);
    }

    let body = match result.text().await {
        Ok(body) => body,
        Err(err) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, format!("Err: {:?}", err));
        }
    };

    (StatusCode::OK, body)
}
