pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20260816_101158_users;
mod m20260816_110942_create_roles_table;
mod m20260816_111007_create_permissions_table;
mod m20260816_111021_create_role_permissions_table;
mod m20260816_111037_create_user_roles_table;
mod m20260816_111046_create_user_permissions_table;
mod m20260816_111110_create_refresh_tokens_table;
mod m20260816_111122_create_password_resets_table;
mod m20260816_111136_create_organizations_table;
mod m20260816_111142_create_organization_members_table;
mod m20260816_111150_create_teams_table;
mod m20260816_111154_create_team_member_table;
mod m20260816_111201_create_projects_table;
mod m20260816_111213_create_project_repositories_table;
mod m20260816_111228_create_project_environment_variables_table;
mod m20260816_111242_create_project_members_table;
mod m20260816_111248_create_project_teams_table;
mod m20260816_111300_create_deployment_table;
mod m20260816_111317_create_notifications_table;
mod m20260819_044939_create_users_profile_table;
mod m20260819_000001_create_organization_invitations_table;
mod m20260819_000002_add_description_and_logo_to_organizations;
mod m20260819_000003_add_role_to_team_members;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20260816_101158_users::Migration),
            Box::new(m20260816_110942_create_roles_table::Migration),
            Box::new(m20260816_111007_create_permissions_table::Migration),
            Box::new(m20260816_111021_create_role_permissions_table::Migration),
            Box::new(m20260816_111037_create_user_roles_table::Migration),
            Box::new(m20260816_111046_create_user_permissions_table::Migration),
            Box::new(m20260816_111110_create_refresh_tokens_table::Migration),
            Box::new(m20260816_111122_create_password_resets_table::Migration),
            Box::new(m20260816_111136_create_organizations_table::Migration),
            Box::new(m20260816_111142_create_organization_members_table::Migration),
            Box::new(m20260816_111150_create_teams_table::Migration),
            Box::new(m20260816_111154_create_team_member_table::Migration),
            Box::new(m20260816_111201_create_projects_table::Migration),
            Box::new(m20260816_111213_create_project_repositories_table::Migration),
            Box::new(m20260816_111228_create_project_environment_variables_table::Migration),
            Box::new(m20260816_111242_create_project_members_table::Migration),
            Box::new(m20260816_111248_create_project_teams_table::Migration),
            Box::new(m20260816_111300_create_deployment_table::Migration),
            Box::new(m20260816_111317_create_notifications_table::Migration),
            Box::new(m20260819_044939_create_users_profile_table::Migration),
            Box::new(m20260819_000001_create_organization_invitations_table::Migration),
            Box::new(m20260819_000002_add_description_and_logo_to_organizations::Migration),
            Box::new(m20260819_000003_add_role_to_team_members::Migration),
        ]
    }
}
