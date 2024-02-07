use crate::eflinttojson::eflint_to_json;
use axum::{extract::Query, http::StatusCode};
use brane_ast::{CompileResult, ParserOptions, Workflow};
use eflint_json::{v0_1_0_srv::Request, DisplayEFlint};
use serde::Deserialize;
use specifications::{data::DataIndex, package::PackageIndex};

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

    println!("from: {:?} to {:?}", conv.from, conv.to);

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
        return match to_wir(body).await {
            Ok(ret) => (StatusCode::OK, ret),
            Err(err) => return (StatusCode::BAD_REQUEST, err.into()),
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
    let req: Request = match serde_json::from_str::<Vec<Request>>(&body) {
        Ok(req) => req.iter().next().unwrap_or_else(|| todo!()).to_owned(),
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let sreq: String = format!("{}", req.display_syntax());

    return Ok(sreq);
}
pub async fn to_wir(body: String) -> Result<String, String> {
    let dindex: DataIndex = match std::panic::catch_unwind(|| {
        brane_shr::utilities::create_data_index_from("./tests/data")
    }) {
        Ok(index) => index,
        Err(err) => return Err(format!("{:?}", err)),
    };

    let pindex: PackageIndex = match std::panic::catch_unwind(|| {
        brane_shr::utilities::create_package_index_from("./tests/packages")
    }) {
        Ok(index) => index,
        Err(err) => return Err(format!("{:?}", err)),
    };

    let workflow: Workflow = match brane_ast::compile_program(
        body.as_bytes(),
        &pindex,
        &dindex,
        &ParserOptions::bscript(),
    ) {
        CompileResult::Workflow(wf, _warns) => wf,
        CompileResult::Err(errs) => todo!(),
        CompileResult::Eof(err) => todo!(),

        CompileResult::Unresolved(_, _) => unreachable!(),
        CompileResult::Program(_, _) => unreachable!(),
    };

    serde_json::to_string_pretty(&workflow).map_err(|err| err.to_string())
}
