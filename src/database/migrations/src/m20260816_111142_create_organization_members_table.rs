use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111142_create_organization_members_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(OrganizationMemberRole::Table)
                    .values([
                        OrganizationMemberRole::Admin,
                        OrganizationMemberRole::Editor,
                        OrganizationMemberRole::Viewer,
                        OrganizationMemberRole::Owner,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganizationMembers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationMembers::OrganizationId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organization_members_organization_id")
                            .from(
                                OrganizationMembers::Table,
                                OrganizationMembers::OrganizationId,
                            )
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_organization_members_user_id")
                            .from(OrganizationMembers::Table, OrganizationMembers::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::Role)
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
                    .primary_key(
                        Index::create()
                            .name("pk_organization_member")
                            .col(OrganizationMembers::OrganizationId)
                            .col(OrganizationMembers::UserId),
                    )
                    .col(
                        ColumnDef::new(OrganizationMembers::JoinedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationMembers::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(OrganizationMemberRole::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum OrganizationMembers {
    Table,
    OrganizationId,
    UserId,
    Role,
    JoinedAt,
}

#[derive(DeriveIden)]
enum Organizations {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
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
