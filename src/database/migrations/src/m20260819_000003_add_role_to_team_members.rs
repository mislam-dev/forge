use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260819_000003_add_role_to_team_members"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(TeamMembers::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(TeamMembers::Role)
                            .string()
                            .not_null()
                            .default("developer"),
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
                    .table(TeamMembers::Table)
                    .drop_column(TeamMembers::Role)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum TeamMembers {
    Table,
    Role,
}
