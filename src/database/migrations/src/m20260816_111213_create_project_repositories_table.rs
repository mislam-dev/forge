use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111213_create_project_repositories_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectRespositoryAuthType::Table)
                    .values([
                        ProjectRespositoryAuthType::None,
                        ProjectRespositoryAuthType::Pat,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ProjectRepositoryStatus::Table)
                    .values([
                        ProjectRepositoryStatus::Connected,
                        ProjectRepositoryStatus::Disconnected,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectRepositories::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectRepositories::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::ProjectId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_project_repositories_project_id")
                            .from(ProjectRepositories::Table, ProjectRepositories::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::RepositoryUrl)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::AuthType)
                            .enumeration(
                                ProjectRespositoryAuthType::Table,
                                [
                                    ProjectRespositoryAuthType::None,
                                    ProjectRespositoryAuthType::Pat,
                                ],
                            )
                            .not_null()
                            .default("none"),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::AccessTokenEncrypted)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ProjectRepositories::Status).enumeration(
                        ProjectRepositoryStatus::Table,
                        [
                            ProjectRepositoryStatus::Connected,
                            ProjectRepositoryStatus::Disconnected,
                        ],
                    ))
                    .col(
                        ColumnDef::new(ProjectRepositories::DefaultBranch)
                            .string()
                            .default("main"),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ProjectRepositories::UpdatedAt)
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
            .drop_table(
                Table::drop()
                    .table(ProjectRepositories::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name(ProjectRespositoryAuthType::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(
                Type::drop()
                    .name(ProjectRepositoryStatus::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProjectRepositories {
    Table,
    Id,
    ProjectId,
    RepositoryUrl,
    AuthType,
    AccessTokenEncrypted,
    DefaultBranch,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum ProjectRepositoryStatus {
    Table,
    Connected,
    Disconnected,
}

#[derive(DeriveIden)]
enum ProjectRespositoryAuthType {
    Table,
    None,
    Pat,
}
