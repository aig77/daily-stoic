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

fn expired_response() -> Response {
    (
        [("HX-Redirect", "/session-expired")],
        Html(ExpiredTemplate.render().unwrap()),
    )
        .into_response()
}

pub struct AuthUser {
    pub email: String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // session lives in request extensions, not passed directly
        let session = Session::from_request_parts(parts, state).await.unwrap();
        match session.get::<String>(EMAIL_KEY).await.unwrap() {
            Some(email) => Ok(AuthUser { email }),
            None => Err(expired_response()),
        }
    }
}

pub enum AdminRejection {
    Expired,
    Forbidden,
}

impl IntoResponse for AdminRejection {
    fn into_response(self) -> Response {
        match self {
            AdminRejection::Expired => expired_response(),
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
            return Err(AdminRejection::Expired);
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
