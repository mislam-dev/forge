use super::dto::request::{CreateOrganizationRequest, UpdateOrganizationRequest};
use super::dto::response::OrganizationResponse;
use super::entities::organization::ActiveModel as OrganizationActiveModel;
use super::repository::OrganizationRepository;
use crate::modules::organization::members::entities::sea_orm_active_enums::OrganizationMemberRole;
use crate::modules::organization::members::repository::OrganizationMembersRepository;
use crate::modules::organization::permissions::role::OrgRole;
use crate::modules::organization::permissions::service::OrgPermissionsService;
use crate::shared::error::AppError;
use chrono::Utc;
use sea_orm::{DatabaseConnection, Set, TransactionTrait};
use uuid::Uuid;

pub struct OrganizationService;

impl OrganizationService {
    pub async fn create_organization(
        db: &DatabaseConnection,
        dto: CreateOrganizationRequest,
    ) -> Result<OrganizationResponse, AppError> {
        let slug = match dto.slug {
            Some(s) if !s.trim().is_empty() => Self::slugify(&s),
            _ => Self::slugify(&dto.name),
        };

        if slug.is_empty() {
            return Err(AppError::BadRequest(
                "Invalid organization name or slug".to_string(),
            ));
        }

        if OrganizationRepository::find_by_slug(db, &slug)
            .await?
            .is_some()
        {
            return Err(AppError::Conflict(
                "An organization with this slug already exists".to_string(),
            ));
        }

        let owner_user_id = dto.owner_user_id.unwrap();
        let org_id = Uuid::new_v4();

        let org_active = OrganizationActiveModel {
            id: Set(org_id),
            name: Set(dto.name),
            slug: Set(slug),
            description: Set(dto.description),
            logo_url: Set(dto.logo_url),
            ..Default::default()
        };

        let txn = db.begin().await?;
        let created_org = OrganizationRepository::create_with_txn(&txn, org_active).await?;

        OrganizationMembersRepository::add_member_with_txn(
            &txn,
            org_id,
            owner_user_id,
            OrganizationMemberRole::Owner,
        )
        .await?;

        txn.commit().await?;

        Ok(OrganizationResponse::from_model(
            created_org,
            Some(owner_user_id),
        ))
    }

    pub async fn get_user_organizations(
        db: &DatabaseConnection,
        user_id: Uuid,
        is_admin: bool,
    ) -> Result<Vec<OrganizationResponse>, AppError> {
        let orgs = if is_admin {
            OrganizationRepository::find_all(db).await?
        } else {
            OrganizationRepository::find_user_organizations(db, user_id).await?
        };

        let mut res = Vec::with_capacity(orgs.len());
        for org in orgs {
            let owner_id = OrganizationRepository::find_owner_id(db, org.id).await?;
            res.push(OrganizationResponse::from_model(org, owner_id));
        }

        Ok(res)
    }

    pub async fn get_organization_by_id(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: Uuid,
        is_admin: bool,
    ) -> Result<OrganizationResponse, AppError> {
        let org = OrganizationRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        if !is_admin {
            let _role =
                OrgPermissionsService::verify_org_role(db, id, user_id, OrgRole::Viewer, is_admin)
                    .await?;
        }

        let owner_id = OrganizationRepository::find_owner_id(db, org.id).await?;
        Ok(OrganizationResponse::from_model(org, owner_id))
    }

    pub async fn update_organization(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: Uuid,
        is_admin: bool,
        dto: UpdateOrganizationRequest,
    ) -> Result<OrganizationResponse, AppError> {
        let org = OrganizationRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let role =
            OrgPermissionsService::verify_org_role(db, id, user_id, OrgRole::Admin, is_admin)
                .await?;

        let mut active: OrganizationActiveModel = org.into();
        let now = Utc::now().into();
        active.updated_at = Set(now);

        if let Some(new_name) = dto.name {
            if role != OrgRole::Owner && !is_admin {
                return Err(AppError::Forbidden(
                    "Only the Organization Owner can rename an organization.".to_string(),
                ));
            }
            active.name = Set(new_name);
        }

        if let Some(new_slug) = dto.slug {
            let slugified = Self::slugify(&new_slug);
            if !slugified.is_empty() {
                let existing = OrganizationRepository::find_by_slug(db, &slugified).await?;
                if let Some(other) = existing {
                    if other.id != id {
                        return Err(AppError::Conflict(
                            "An organization with this slug already exists".to_string(),
                        ));
                    }
                }
                active.slug = Set(slugified);
            }
        }

        if let Some(desc) = dto.description {
            active.description = Set(Some(desc));
        }

        if let Some(logo) = dto.logo_url {
            active.logo_url = Set(Some(logo));
        }

        let updated_org = OrganizationRepository::update(db, active).await?;
        let owner_id = OrganizationRepository::find_owner_id(db, updated_org.id).await?;

        Ok(OrganizationResponse::from_model(updated_org, owner_id))
    }

    pub async fn delete_organization(
        db: &DatabaseConnection,
        id: Uuid,
        user_id: Uuid,
        is_admin: bool,
    ) -> Result<(), AppError> {
        let _org = OrganizationRepository::find_by_id(db, id)
            .await?
            .ok_or_else(|| AppError::NotFound("Organization not found".to_string()))?;

        let role =
            OrgPermissionsService::verify_org_role(db, id, user_id, OrgRole::Owner, is_admin)
                .await?;
        OrgPermissionsService::enforce_delete_permission(role, is_admin)?;

        OrganizationRepository::delete(db, id).await?;
        Ok(())
    }

    fn slugify(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    fn setup_mock_db() -> DatabaseConnection {
        MockDatabase::new(DatabaseBackend::Postgres).into_connection()
    }

    #[test]
    fn test_slugify() {
        assert_eq!(OrganizationService::slugify("Acme Corp!"), "acme-corp");
        assert_eq!(
            OrganizationService::slugify("  My---Cool--Org  "),
            "my-cool-org"
        );
        assert_eq!(
            OrganizationService::slugify("FORGE_PLATFORM"),
            "forge-platform"
        );
    }

    #[tokio::test]
    async fn test_get_organization_by_id_not_found() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let result = OrganizationService::get_organization_by_id(&db, org_id, user_id, false).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_organization_not_found() {
        let db = setup_mock_db();
        let org_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let result = OrganizationService::delete_organization(&db, org_id, user_id, true).await;
        assert!(result.is_err());
    }
}
