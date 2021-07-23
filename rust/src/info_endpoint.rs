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
            "app_name": info::PKG_NAME,
            "version": info::PKG_VERSION,
            "profile": info::PROFILE,
            "git_head_ref": info::GIT_HEAD_REF,
            "git_commit_hash": info::GIT_COMMIT_HASH,
            "build_time_utc": info::BUILT_TIME_UTC,
        })
    });
    HttpResponse::Ok().json(&info)
}
