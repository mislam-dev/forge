use axum::{
    extract::{FromRequestParts, Request, State},
    http::Uri,
    middleware::Next,
    response::Response,
};
use axum_limit::LimitPerMinute;

use crate::{app::state::AppState, shared::error::AppError};

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let (mut parts, body) = req.into_parts();

    let _limit = LimitPerMinute::<15, Uri>::from_request_parts(&mut parts, &state)
        .await
        .map_err(|e| AppError::too_many_requests(e.to_string()))?;

    Ok(next.run(Request::from_parts(parts, body)).await)
}
