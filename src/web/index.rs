use askama::Template;
use axum::{
    body::Body,
    http::{Response, StatusCode},
    routing::get,
    Router,
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(self::get::index))
        .route("/style", get(self::get::style))
}

mod get {
    use super::*;

    pub async fn index() -> IndexTemplate {
        IndexTemplate {}
    }

    pub async fn style() -> axum::response::Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "text/css")
            .body(Body::from(include_str!("../../static/dist.css")))
            .unwrap()
    }
}
