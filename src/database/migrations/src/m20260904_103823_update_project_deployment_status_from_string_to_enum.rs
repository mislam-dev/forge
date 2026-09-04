use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260904_103823_update_project_deployment_status_from_string_to_enum"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .alter_table(
                Table::alter()
                    .table(Deployments::Table)
                    .drop_column(Deployments::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(DeploymentStatus::Table)
                    .values([
                        DeploymentStatus::Queued,
                        DeploymentStatus::Building,
                        DeploymentStatus::Deploying,
                        DeploymentStatus::Running,
                        DeploymentStatus::Failed,
                        DeploymentStatus::Success,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Deployments::Table)
                    .add_column(
                        ColumnDef::new(Deployments::Status)
                            .enumeration(
                                DeploymentStatus::Table,
                                [
                                    DeploymentStatus::Queued,
                                    DeploymentStatus::Building,
                                    DeploymentStatus::Deploying,
                                    DeploymentStatus::Running,
                                    DeploymentStatus::Failed,
                                    DeploymentStatus::Success,
                                ],
                            )
                            .not_null()
                            .default(Expr::value("queued")),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Deployments::Table)
                    .drop_column(Deployments::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(DeploymentStatus::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Deployments::Table)
                    .add_column(
                        ColumnDef::new(Deployments::Status)
                            .string()
                            .not_null()
                            .to_owned(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Deployments {
    Table,
    Status,
}

#[derive(DeriveIden)]
enum DeploymentStatus {
    Table,
    Queued,
    Building,
    Deploying,
    Running,
    Failed,
    Success,
}
