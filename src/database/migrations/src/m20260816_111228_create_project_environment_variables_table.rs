use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111228_create_project_environment_variables_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(EnviromentType::Table)
                    .values([
                        EnviromentType::Development,
                        EnviromentType::Production,
                        EnviromentType::Staging,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ProjectEnvironmentVariables::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::ProjectId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-project_environment_variables-project_id")
                            .from(
                                ProjectEnvironmentVariables::Table,
                                ProjectEnvironmentVariables::ProjectId,
                            )
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::Environment)
                            .enumeration(
                                EnviromentType::Table,
                                [
                                    EnviromentType::Development,
                                    EnviromentType::Production,
                                    EnviromentType::Staging,
                                ],
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::Key)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::ValueEncrypted)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::IsSecret)
                            .boolean()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("now()")),
                    )
                    .col(
                        ColumnDef::new(ProjectEnvironmentVariables::UpdatedAt)
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
                    .table(ProjectEnvironmentVariables::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(Type::drop().name(EnviromentType::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum ProjectEnvironmentVariables {
    Table,
    Id,
    ProjectId,
    Environment,
    Key,
    ValueEncrypted,
    IsSecret,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum EnviromentType {
    #[sea_orm(iden = "project_environment_variables_environment")]
    Table,
    Development,
    Staging,
    Production,
}
