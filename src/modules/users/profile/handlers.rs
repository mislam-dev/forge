use axum::extract::{Path, State};
use axum::http::StatusCode;

use super::dto::request::UpdateUserProfileDto;
use super::dto::response::UserProfileResponse;
use super::service::UserProfileService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::{
    error::AppError, response::ApiResponse, utils::IdParams, validation::JsonValidate,
};

pub async fn get_profile(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
) -> Result<ApiResponse<UserProfileResponse>, AppError> {
    let profile = UserProfileService::get_profile(&state.db, id.0).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("User profile retrieved successfully".to_string())
        .body(Some(profile)))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Path(id): Path<IdParams>,
    jwt_claims: JwtClaims,
    JsonValidate(payload): JsonValidate<UpdateUserProfileDto>,
) -> Result<ApiResponse<UserProfileResponse>, AppError> {
    let is_admin = jwt_claims
        .roles
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin"));
    if jwt_claims.sub != id.0 && !is_admin {
        return Err(AppError::Forbidden(
            "You are not authorized to update this profile".to_string(),
        ));
    }

    let profile = UserProfileService::update_profile(&state.db, id.0, payload).await?;
    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("User profile updated successfully".to_string())
        .body(Some(profile)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;
    use uuid::Uuid;

    fn setup_mock_state() -> AppState {
        unsafe {
            std::env::set_var("DATABASE_URL", "postgres://user:pass@localhost:5432/testdb");
            std::env::set_var("JWT_SECRET", "test_secret_key_12345_67890_super_secret");
            std::env::set_var("MASTER_ENCRYPTION_KEY", "0123456789abcdef0123456789abcdef");
        }
        let config = AppConfig::load().expect("Test AppConfig must load successfully");
        AppState::mock(config)
    }

    #[tokio::test]
    async fn test_get_profile_handler() {
        let state = setup_mock_state();
        let id = IdParams(Uuid::new_v4());
        let result = get_profile(State(state), Path(id)).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_profile_handler_forbidden_other_user() {
        let state = setup_mock_state();
        let user_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        let claims = JwtClaims {
            sub: user_id,
            email: "user@example.com".to_string(),
            roles: vec!["User".to_string()],
            permissions: vec![],
            iat: 100000,
            exp: 200000,
        };
        let payload = UpdateUserProfileDto {
            first_name: Some("Test".to_string()),
            last_name: None,
            phone: None,
            date_of_birth: None,
            gender: None,
            image: None,
        };
        let result = update_profile(
            State(state),
            Path(IdParams(other_user_id)),
            claims,
            JsonValidate(payload),
        )
        .await;
        assert!(result.is_err());
    }
}
