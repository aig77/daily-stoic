use super::sessions::{EMAIL_KEY, Session};
use crate::AppState;

use askama::Template;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{Html, IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "auth/expired.html")]
pub struct ExpiredTemplate;

pub struct AuthUser {
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Html<String>;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // session lives in request extensions, not passed directly
        let session = Session::from_request_parts(parts, state).await.unwrap();
        match session.get::<String>(EMAIL_KEY).await.unwrap() {
            Some(email) => Ok(AuthUser { email }),
            None => Err(Html(ExpiredTemplate.render().unwrap())),
        }
    }
}

pub enum AdminRejection {
    Expired(Html<String>),
    Forbidden,
}

impl IntoResponse for AdminRejection {
    fn into_response(self) -> Response {
        match self {
            AdminRejection::Expired(html) => html.into_response(),
            AdminRejection::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

pub struct AdminUser {
    pub email: String,
}

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = AdminRejection;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await.unwrap();
        let Some(email) = session.get::<String>(EMAIL_KEY).await.unwrap() else {
            return Err(AdminRejection::Expired(Html(
                ExpiredTemplate.render().unwrap(),
            )));
        };

        if let Some(user) = state.db.users.get(&email).await
            && user.is_admin == 1
        {
            Ok(AdminUser { email })
        } else {
            Err(AdminRejection::Forbidden)
        }
    }
}
