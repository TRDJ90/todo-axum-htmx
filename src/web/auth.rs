use crate::users::{AuthSession, Credentials};
use askama::Template;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    message: Option<String>,
    next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(self::post::login))
        .route("/login", get(self::get::login))
        .route("/logout", get(self::get::logout))
}

mod post {

    use super::*;

    pub async fn login(
        mut auth_session: AuthSession,
        Form(credentials): Form<Credentials>,
    ) -> impl IntoResponse {
        let user = match auth_session.authenticate(credentials.clone()).await {
            Ok(Some(user)) => user,
            Ok(None) => {
                return LoginTemplate {
                    message: Some("Invalid credentials".to_string()),
                    next: credentials.next,
                }
                .into_response()
            }
            Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        if let Some(ref next) = credentials.next {
            Redirect::to(next).into_response()
        } else {
            Redirect::to("/todo").into_response()
        }
    }
}

mod get {
    use super::*;

    pub async fn login(Query(NextUrl { next }): Query<NextUrl>) -> LoginTemplate {
        LoginTemplate {
            message: None,
            next,
        }
    }

    pub async fn logout(mut auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.logout() {
            Ok(_) => Redirect::to("/").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
