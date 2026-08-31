use crate::shared::error::AppError;
use axum::extract::FromRequestParts;
use uuid::Uuid;

pub struct OrgIdHeader(pub Uuid);

impl<S> FromRequestParts<S> for OrgIdHeader
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let org_id_header = parts.headers.get("Organization-ID").ok_or_else(|| {
            AppError::BadRequest("Organization-ID header is required".to_string())
        })?;

        let org_id_str = org_id_header.to_str().map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header encoding".to_string())
        })?;

        let org_id = Uuid::parse_str(org_id_str).map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header: must be a valid UUID".to_string())
        })?;

        Ok(Self(org_id))
    }
}
