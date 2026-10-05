use super::dto::{BuildLogResponse, LogSearchQuery};
use super::service::BuildLogsService;
use crate::app::state::AppState;
use crate::modules::projects::deployments::DeploymentStatus;
use crate::modules::projects::deployments::repository::DeploymentsRepository;
use crate::modules::projects::extractors::{OptionalOrgViewer, OrgValidationOptional};
use crate::modules::projects::logs::LogStreamConsumer;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use axum::response::sse::Event;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response, sse::Sse},
};
use std::convert::Infallible;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::Stream;
use tokio_stream::wrappers::ReceiverStream;
use uuid::Uuid;

pub async fn get_logs(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<ApiResponse<BuildLogResponse>, AppError> {
    let logs =
        BuildLogsService::get_logs(&state.db, &state.loki, org_id, id, deployment_id).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Build logs retrieved successfully.".to_string())
        .body(Some(logs)))
}

pub async fn stream_logs(
    State(state): State<AppState>,
    OrgValidationOptional(_, _, _): OptionalOrgViewer,
    Path((_id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    let deployment = DeploymentsRepository::find_by_id(&state.db, deployment_id).await?;
    if let Some(deployment) = deployment
        && deployment.status != DeploymentStatus::Running
    {
        return Err(AppError::NotFound(format!(
            "There is no running build-logs to stream for deployment {}",
            deployment_id
        )));
    }

    let (tx, rx) = mpsc::channel::<Result<Event, Infallible>>(100);

    let logs_stream_consumer = LogStreamConsumer::new();
    let _s = logs_stream_consumer
        .start(state, deployment_id.to_string(), tx)
        .await;

    let stream: ReceiverStream<Result<Event, Infallible>> = ReceiverStream::new(rx);

    Ok(Sse::new(stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15))))
}

pub async fn download_logs(
    State(state): State<AppState>,
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
) -> Result<Response, AppError> {
    let text =
        BuildLogsService::download_logs(&state.db, &state.loki, org_id, id, deployment_id).await?;

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
    OrgValidationOptional(_, org_id, _): OptionalOrgViewer,
    Path((id, deployment_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<LogSearchQuery>,
) -> Result<ApiResponse<BuildLogResponse>, AppError> {
    let logs =
        BuildLogsService::search_logs(&state.db, &state.loki, org_id, id, deployment_id, query)
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
