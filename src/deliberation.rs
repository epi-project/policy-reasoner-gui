use axum::http::StatusCode;

use axum_extra::extract::cookie::PrivateCookieJar;

// Exec task request
pub async fn post_exec_task(jar: PrivateCookieJar, body: String) -> (StatusCode, String) {
    let deliberation_auth_token = match jar.get("reasoner_deliberation_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };
    let client = reqwest::Client::new();
    let result = match client
        .post("http://localhost:3030/v1/deliberation/execute-task")
        .body(body)
        .header(
            "Authorization",
            format!("Bearer {}", deliberation_auth_token.value()),
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

// Access data request
pub async fn post_access_data(jar: PrivateCookieJar, body: String) -> (StatusCode, String) {
    let deliberation_auth_token = match jar.get("reasoner_deliberation_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };
    println!("Bearer {}", deliberation_auth_token.value());
    let client = reqwest::Client::new();
    let result = match client
        .post("http://localhost:3030/v1/deliberation/access-data")
        .body(body)
        .header(
            "Authorization",
            format!("Bearer {}", deliberation_auth_token.value()),
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

// Validate workflow request
pub async fn post_validate_workflow(jar: PrivateCookieJar, body: String) -> (StatusCode, String) {
    let deliberation_auth_token = match jar.get("reasoner_deliberation_auth") {
        Some(data) => data,
        None => {
            return (StatusCode::UNAUTHORIZED, "".into());
        }
    };
    let client = reqwest::Client::new();
    let result = match client
        .post("http://localhost:3030/v1/deliberation/execute-workflow")
        .body(body)
        .header(
            "Authorization",
            format!("Bearer {}", deliberation_auth_token.value()),
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
