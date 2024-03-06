use axum::extract::Query;
use axum::http::StatusCode;
use eflint_json::v0_1_0_srv::Request;
use eflint_json::DisplayEFlint;
use serde::Deserialize;

use crate::bstowir::{bs_to_wir, IndexSource};
use crate::eflinttojson::eflint_to_json;

#[derive(Deserialize)]
pub struct ConvQuery {
    pub from: CodeFormat,
    pub to: CodeFormat,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CodeFormat {
    EFlintJson,
    EFlint,
    WIR,
    BraneScript,
}

pub async fn post_conv(Query(conv): Query<ConvQuery>, body: String) -> (StatusCode, String) {
    if conv.from == conv.to {
        return (
            StatusCode::BAD_REQUEST,
            "From and into can't be the same format".into(),
        );
    }

    if conv.from == CodeFormat::EFlint && conv.to == CodeFormat::EFlintJson {
        return match to_eflint_json(body).await {
            Ok(ret) => (StatusCode::OK, ret),
            Err(err) => return (StatusCode::BAD_REQUEST, err.into()),
        };
    } else if conv.from == CodeFormat::EFlintJson && conv.to == CodeFormat::EFlint {
        return match to_eflint(body).await {
            Ok(ret) => (StatusCode::OK, ret),
            Err(err) => return (StatusCode::BAD_REQUEST, err.into()),
        };
    } else if conv.from == CodeFormat::BraneScript && conv.to == CodeFormat::WIR {
        return match bs_to_wir(
            IndexSource::LocalTest("./tests/packages".into()),
            IndexSource::LocalTest("./tests/data".into()),
            body,
        )
        .await
        {
            Ok(ret) => (StatusCode::OK, ret),
            Err((code, msg)) => return (code, msg),
        };
    }

    return (StatusCode::BAD_REQUEST, "Invalid conversion".into());
}

pub async fn to_eflint_json(body: String) -> Result<String, String> {
    match eflint_to_json(body).await {
        Ok(req) => Ok(req),
        Err(err) => {
            return Err(err.to_string());
        }
    }
}
pub async fn to_eflint(body: String) -> Result<String, String> {
    let req: Request = match serde_json::from_str::<Request>(&body) {
        Ok(req) => req,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let phrases: Vec<String> = req
        .into_phrases()
        .phrases
        .iter()
        .map(|p| p.display_syntax().to_string())
        .collect();

    let sreq = phrases.join("\n");

    return Ok(sreq.trim().into());
}
