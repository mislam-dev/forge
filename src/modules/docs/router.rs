use crate::app::state::AppState;
use axum::{Router, http::header, response::IntoResponse, routing::get};
use utoipa_swagger_ui::{Config, SwaggerUi, Url};

const OPENAPI_SPEC: &str = include_str!("../../../docs/system/05-api/openapi.yaml");

async fn openapi_yaml_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/x-yaml; charset=utf-8")],
        OPENAPI_SPEC,
    )
}

pub fn docs_router() -> Router<AppState> {
    let config = Config::new([Url::new("Forge Platform API", "/docs/openapi.yaml")]);
    let swagger = SwaggerUi::new("/docs").config(config);

    Router::new()
        .route("/docs/openapi.yaml", get(openapi_yaml_handler))
        .merge(swagger)
}
