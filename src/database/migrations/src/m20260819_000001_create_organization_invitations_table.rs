use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260819_000001_create_organization_invitations_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(OrganizationInvitationsStatus::Table)
                    .values([
                        OrganizationInvitationsStatus::Accepted,
                        OrganizationInvitationsStatus::Pending,
                        OrganizationInvitationsStatus::Rejected,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganizationInvitations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationInvitations::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organization_invitations_organization_id")
                            .from(
                                OrganizationInvitations::Table,
                                OrganizationInvitations::OrganizationId,
                            )
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::Email)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::Role)
                            .enumeration(
                                OrganizationMemberRole::Table,
                                [
                                    OrganizationMemberRole::Admin,
                                    OrganizationMemberRole::Editor,
                                    OrganizationMemberRole::Viewer,
                                    OrganizationMemberRole::Owner,
                                ],
                            )
                            .default("viewer"),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::Token)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::Status)
                            .enumeration(
                                OrganizationInvitationsStatus::Table,
                                [
                                    OrganizationInvitationsStatus::Pending,
                                    OrganizationInvitationsStatus::Accepted,
                                    OrganizationInvitationsStatus::Rejected,
                                ],
                            )
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::ExpiresAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(OrganizationInvitations::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OrganizationInvitations::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum OrganizationInvitations {
    Table,
    Id,
    OrganizationId,
    Email,
    Role,
    Token,
    Status,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OrganizationMemberRole {
    Table,
    Viewer,
    Editor,
    Admin,
    Owner,
}

#[derive(DeriveIden)]
enum OrganizationInvitationsStatus {
    Table,
    Pending,
    Accepted,
    Rejected,
}
