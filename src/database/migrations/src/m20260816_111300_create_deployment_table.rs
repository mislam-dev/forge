use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111300_create_deployment_table"
    }
}

const FK_DEPLOYMENTS_PROJECT_ID: &str = "fk_deployments_project_id";
const FK_DEPLOYMENTS_TRIGGERED_BY: &str = "fk_deployments_triggered_by";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Deployments::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Deployments::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Deployments::ProjectId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_DEPLOYMENTS_PROJECT_ID)
                            .from(Deployments::Table, Deployments::ProjectId)
                            .to(Projects::Table, Projects::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Deployments::TriggeredBy).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_DEPLOYMENTS_TRIGGERED_BY)
                            .from(Deployments::Table, Deployments::TriggeredBy)
                            .to(Users::Table, Users::Id),
                    )
                    .col(
                        ColumnDef::new(Deployments::Branch)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Deployments::CommitHash)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Deployments::Status).string().not_null())
                    .col(ColumnDef::new(Deployments::BuildDuration).integer().null())
                    .col(ColumnDef::new(Deployments::DeployDuration).integer().null())
                    .col(ColumnDef::new(Deployments::ErrorMessage).text().null())
                    .col(
                        ColumnDef::new(Deployments::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Deployments::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .check(Expr::col(Deployments::Status).is_in([
                        "Queued",
                        "Building",
                        "Deploying",
                        "Running",
                        "Failed",
                        "Success",
                    ]))
                    .to_owned(),
            )
            .await?;

        // FK lookups will happen constantly (deployments for a project,
        // deployments triggered by a user) — index both.
        manager
            .create_index(
                Index::create()
                    .name("idx_deployments_project_id")
                    .table(Deployments::Table)
                    .col(Deployments::ProjectId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_deployments_triggered_by")
                    .table(Deployments::Table)
                    .col(Deployments::TriggeredBy)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Deployments::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Deployments {
    Table,
    Id,
    ProjectId,
    TriggeredBy,
    Branch,
    CommitHash,
    Status,
    BuildDuration,
    DeployDuration,
    ErrorMessage,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Projects {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
