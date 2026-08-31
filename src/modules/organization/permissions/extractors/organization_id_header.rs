use crate::shared::error::AppError;
use axum::extract::FromRequestParts;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrgIdHeaderOptional(pub Option<Uuid>);

impl<S> FromRequestParts<S> for OrgIdHeaderOptional
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let Some(org_id_header) = parts.headers.get("Organization-ID") else {
            return Ok(Self(None));
        };

        let org_id_str = org_id_header.to_str().map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header encoding".to_string())
        })?;

        let org_id = Uuid::parse_str(org_id_str).map_err(|_| {
            AppError::BadRequest("Invalid Organization-ID header: must be a valid UUID".to_string())
        })?;

        Ok(Self(Some(org_id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;

    #[tokio::test]
    async fn test_org_id_header_valid() {
        let expected_uuid = Uuid::new_v4();
        let req = Request::builder()
            .header("Organization-ID", expected_uuid.to_string())
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeader::from_request_parts(&mut parts, &()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, expected_uuid);
    }

    #[tokio::test]
    async fn test_org_id_header_missing() {
        let req = Request::builder().body(()).unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeader::from_request_parts(&mut parts, &()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => assert_eq!(msg, "Organization-ID header is required"),
            other => panic!("Expected BadRequest error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_org_id_header_invalid_uuid() {
        let req = Request::builder()
            .header("Organization-ID", "invalid-uuid-string")
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeader::from_request_parts(&mut parts, &()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Invalid Organization-ID header: must be a valid UUID")
            }
            other => panic!("Expected BadRequest error, got: {:?}", other),
        }
    }

    #[tokio::test]
    async fn test_org_id_header_optional_present_valid() {
        let expected_uuid = Uuid::new_v4();
        let req = Request::builder()
            .header("Organization-ID", expected_uuid.to_string())
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeaderOptional::from_request_parts(&mut parts, &()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, Some(expected_uuid));
    }

    #[tokio::test]
    async fn test_org_id_header_optional_absent() {
        let req = Request::builder().body(()).unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeaderOptional::from_request_parts(&mut parts, &()).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, None);
    }

    #[tokio::test]
    async fn test_org_id_header_optional_invalid_uuid() {
        let req = Request::builder()
            .header("Organization-ID", "invalid-uuid-string")
            .body(())
            .unwrap();

        let (mut parts, _) = req.into_parts();
        let result = OrgIdHeaderOptional::from_request_parts(&mut parts, &()).await;

        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "Invalid Organization-ID header: must be a valid UUID")
            }
            other => panic!("Expected BadRequest error, got: {:?}", other),
        }
    }
}
