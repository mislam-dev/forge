use super::dto::{BuildLogResponse, LogSearchQuery};
use super::service::BuildLogsService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::modules::projects::logs::LogStreamConsumer;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response, sse::Sse},
};

use axum::response::sse::Event;
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::{self as stream, Stream};
use uuid::Uuid;

pub async fn get_logs(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<BuildLogResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let logs =
        BuildLogsService::get_logs(&state.db, claims.sub, is_admin, id, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Build logs retrieved successfully.".to_string())
        .body(Some(logs)))
}

pub async fn stream_logs(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((_id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // let _is_admin = claims
    //     .roles
    //     .iter()
    //     .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(100);

    let logs_stream_consumer = LogStreamConsumer::new();
    let _s = logs_stream_consumer
        .start(state, deployment_id.to_string(), tx)
        .await;

    let stream = ReceiverStream::new(rx);

    Ok(Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))))
}

pub async fn download_logs(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let text =
        BuildLogsService::download_logs(&state.db, claims.sub, is_admin, id, deployment_id).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"deployment-{}.log\"", deployment_id)
            .parse()
            .unwrap(),
    );

    Ok((StatusCode::OK, headers, text).into_response())
}

pub async fn search_logs(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<LogSearchQuery>,
) -> Result<ApiResponse<BuildLogResponse>, AppError> {
    let is_admin = claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let logs =
        BuildLogsService::search_logs(&state.db, claims.sub, is_admin, id, deployment_id, query)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Build log search completed successfully.".to_string())
        .body(Some(logs)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_search_query_construction() {
        let query = LogSearchQuery {
            q: "error".to_string(),
            page: Some(1),
            per_page: Some(10),
        };
        assert_eq!(query.q, "error");
    }
}
