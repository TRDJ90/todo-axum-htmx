use anyhow::Result;
use askama::Template;
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    http::{Response, StatusCode},
    routing::get,
    BoxError,
};
use axum_login::{
    login_required,
    tower_sessions::{Expiry, MemoryStore, SessionManagerLayer},
    AuthManagerLayerBuilder,
};
use sqlx::SqlitePool;
use time::Duration;
use tower::ServiceBuilder;

use crate::{
    users::Backend,
    web::{auth, protected},
};

pub struct App {
    database: SqlitePool,
}

impl App {
    pub async fn new() -> Result<Self> {
        let db = SqlitePool::connect(":memory:").await?;
        sqlx::migrate!().run(&db).await?;

        Ok(Self { database: db })
    }

    pub async fn serve(self) -> Result<()> {
        let session_store = MemoryStore::default();
        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::days(1)));

        let backend = Backend::new(self.database);
        let auth_service = ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::BAD_REQUEST
            }))
            .layer(AuthManagerLayerBuilder::new(backend, session_layer).build());

        let app = protected::router()
            .route_layer(login_required!(Backend, login_url = "/login"))
            .route("/", get(index))
            .route("/style", get(style))
            .merge(auth::router())
            .layer(auth_service);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}

async fn style() -> axum::response::Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(include_str!("../../static/dist.css")))
        .unwrap()
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

async fn index() -> IndexTemplate {
    IndexTemplate {}
}
