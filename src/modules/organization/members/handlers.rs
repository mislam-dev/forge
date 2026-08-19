use axum::{
    extract::{Path, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;

use super::dto::request::{InviteMemberRequest, UpdateMemberRoleRequest};
use super::dto::response::{InvitationResponse, MemberResponse};
use super::service::OrganizationMembersService;
use crate::app::state::AppState;
use crate::modules::auth::token::JwtClaims;
use crate::shared::error::AppError;
use crate::shared::response::ApiResponse;
use crate::shared::validation::JsonValidate;

#[derive(Deserialize)]
pub struct OrgMemberPathParams {
    pub id: Uuid,
    pub user_id: Uuid,
}

pub async fn invite(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
    JsonValidate(payload): JsonValidate<InviteMemberRequest>,
) -> Result<ApiResponse<InvitationResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let res =
        OrganizationMembersService::invite_member(&state.db, id, claims.sub, is_admin, payload)
            .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::CREATED)
        .message("Invitation sent successfully.".to_string())
        .body(Some(res)))
}

pub async fn list_invitations(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<InvitationResponse>>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let res =
        OrganizationMembersService::list_invitations(&state.db, id, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Pending invitations retrieved.".to_string())
        .body(Some(res)))
}

pub async fn accept_invitation(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(token): Path<String>,
) -> Result<ApiResponse<MemberResponse>, AppError> {
    let res = OrganizationMembersService::accept_invitation(&state.db, &token, claims.sub).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Invitation accepted.".to_string())
        .body(Some(res)))
}

pub async fn list_members(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(id): Path<Uuid>,
) -> Result<ApiResponse<Vec<MemberResponse>>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let res = OrganizationMembersService::list_members(&state.db, id, claims.sub, is_admin).await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Organization members retrieved.".to_string())
        .body(Some(res)))
}

pub async fn update_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(params): Path<OrgMemberPathParams>,
    JsonValidate(payload): JsonValidate<UpdateMemberRoleRequest>,
) -> Result<ApiResponse<MemberResponse>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    let res = OrganizationMembersService::update_member_role(
        &state.db,
        params.id,
        params.user_id,
        claims.sub,
        is_admin,
        payload,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Member role updated successfully.".to_string())
        .body(Some(res)))
}

pub async fn remove_member(
    State(state): State<AppState>,
    claims: JwtClaims,
    Path(params): Path<OrgMemberPathParams>,
) -> Result<ApiResponse<()>, AppError> {
    let is_admin = claims
        .role
        .iter()
        .any(|r| r.eq_ignore_ascii_case("admin") || r.eq_ignore_ascii_case("system_admin"));
    OrganizationMembersService::remove_member(
        &state.db,
        params.id,
        params.user_id,
        claims.sub,
        is_admin,
    )
    .await?;

    Ok(ApiResponse::new()
        .status(StatusCode::OK)
        .message("Member removed from organization.".to_string()))
}
