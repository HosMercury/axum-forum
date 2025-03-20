use crate::models::user::User;
use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn auth_middleware(session: Session, request: Request, next: Next) -> impl IntoResponse {
    match session
        .get::<User>("auth_user")
        .await
        .expect("Session getting user failed")
    {
        Some(_) => next.run(request).await,
        None => Redirect::to("/login").into_response(),
    }
}
