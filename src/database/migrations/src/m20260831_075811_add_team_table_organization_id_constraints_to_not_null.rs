use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260831_075811_add_team_table_organization_id_constraints_to_not_null"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .modify_column(ColumnDef::new(Teams::OrganizationId).uuid().not_null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Teams::Table)
                    .modify_column(ColumnDef::new(Teams::OrganizationId).uuid().null())
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
#[derive(DeriveIden)]
enum Teams {
    Table,
    OrganizationId,
}
