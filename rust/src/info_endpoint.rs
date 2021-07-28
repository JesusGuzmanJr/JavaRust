use actix_web::{get, HttpResponse};
use once_cell::sync::OnceCell;

static INFO: OnceCell<serde_json::Value> = OnceCell::new();

// https://docs.rs/built/0.5.1/built/
mod info {
    include!(concat!(env!("OUT_DIR"), "/build_info.rs"));
}

#[get("/info")]
async fn get_info() -> HttpResponse {
    let info = INFO.get_or_init(|| {
        serde_json::json!( {
            "appName": info::PKG_NAME,
            "version": info::PKG_VERSION,
            "profile": info::PROFILE,
            "gitHeadRef": info::GIT_HEAD_REF,
            "gitCommitHash": info::GIT_COMMIT_HASH,
            "buildTimeUtc": info::BUILT_TIME_UTC,
        })
    });
    HttpResponse::Ok().json(&info)
}
