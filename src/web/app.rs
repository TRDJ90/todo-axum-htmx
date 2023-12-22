use anyhow::Result;
use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    response::{Html, IntoResponse},
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
            .route(
                "/style",
                get(|| async { include_str!("../../static/dist.css") }),
            )
            .merge(auth::router())
            .layer(auth_service);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        axum::serve(listener, app.into_make_service()).await?;

        Ok(())
    }
}
