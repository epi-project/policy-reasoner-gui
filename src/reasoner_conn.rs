use axum::http::StatusCode;
use axum_extra::extract::cookie::PrivateCookieJar;

// Get connector info
pub async fn get_reasoner_connector_info(jar: PrivateCookieJar) -> (StatusCode, String) {
    let policy_auth_token = match jar.get("reasoner_policy_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };

    let client = reqwest::Client::new();
    let result = match client
        .get("http://localhost:3030/v1/management/reasoner-connector-context")
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
