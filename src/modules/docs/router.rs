use axum::{
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
use utoipa_swagger_ui::{Config, SwaggerUi, Url};
use crate::app::state::AppState;

const OPENAPI_SPEC: &str = include_str!("./openapi.yaml");

async fn openapi_yaml_handler() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/x-yaml; charset=utf-8")], OPENAPI_SPEC)
}

pub fn docs_router() -> Router<AppState> {
    let config = Config::new([Url::new("Forge Platform API", "/docs/openapi.yaml")]);
    let swagger = SwaggerUi::new("/docs").config(config);

    Router::new()
        .route("/docs/openapi.yaml", get(openapi_yaml_handler))
        .merge(swagger)
}
