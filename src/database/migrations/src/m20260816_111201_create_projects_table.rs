use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111201_create_projects_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectTypes::Table)
                    .values([ProjectTypes::Files, ProjectTypes::Repo])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectRuntime::Table)
                    .values([
                        ProjectRuntime::NodeJs,
                        ProjectRuntime::Python,
                        ProjectRuntime::Go,
                        ProjectRuntime::Static,
                    ])
                    .to_owned(),
            )
            .await?;
        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectStatus::Table)
                    .values([
                        ProjectStatus::Active,
                        ProjectStatus::Inactive,
                        ProjectStatus::Archived,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Projects::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Projects::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Projects::OrganizationId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_organization_id")
                            .from(Projects::Table, Projects::OrganizationId)
                            .to(Organizations::Table, Organizations::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Projects::OwnerId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_projects_owner_id")
                            .from(Projects::Table, Projects::OwnerId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Projects::Name).string().not_null())
                    .col(ColumnDef::new(Projects::Description).string())
                    .col(
                        ColumnDef::new(Projects::ProjectType)
                            .enumeration(
                                ProjectTypes::Table,
                                [ProjectTypes::Files, ProjectTypes::Repo],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Projects::Runtime)
                            .enumeration(
                                ProjectRuntime::Table,
                                [
                                    ProjectRuntime::NodeJs,
                                    ProjectRuntime::Python,
                                    ProjectRuntime::Go,
                                    ProjectRuntime::Static,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Projects::Status)
                            .enumeration(
                                ProjectStatus::Table,
                                [
                                    ProjectStatus::Active,
                                    ProjectStatus::Inactive,
                                    ProjectStatus::Archived,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Projects::Port)
                            .integer()
                            .not_null()
                            .default(3000),
                    )
                    .col(
                        ColumnDef::new(Projects::HealthCheckUrl)
                            .string()
                            .default("/health"),
                    )
                    .col(
                        ColumnDef::new(Projects::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(Projects::UpdatedAt)
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
        // Drop the projects table first, then drop the associated enum types.
        manager
            .drop_table(Table::drop().table(Projects::Table).if_exists().to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(ProjectTypes::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(ProjectRuntime::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(ProjectStatus::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
    OrganizationId,
    OwnerId,
    Name,
    Description,
    ProjectType,
    Runtime,
    Port,
    HealthCheckUrl,
    Status,
    CreatedAt,
    UpdatedAt,
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
enum ProjectTypes {
    Table,
    Repo,
    Files,
}

#[derive(DeriveIden)]
enum ProjectRuntime {
    Table,
    NodeJs,
    Python,
    Go,
    Static,
}

#[derive(DeriveIden)]
enum ProjectStatus {
    Table,
    Active,
    Inactive,
    Archived,
}
