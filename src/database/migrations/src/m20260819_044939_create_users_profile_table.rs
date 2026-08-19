use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260819_044939_create_users_profile_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Gender::Table)
                    .values([Gender::Male, Gender::Female, Gender::Other])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserProfile::Table)
                    .col(
                        ColumnDef::new(UserProfile::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(UserProfile::FirstName).string())
                    .col(ColumnDef::new(UserProfile::LastName).string())
                    .col(ColumnDef::new(UserProfile::Phone).string())
                    .col(ColumnDef::new(UserProfile::DateOfBirth).date())
                    .col(
                        ColumnDef::new(UserProfile::Gender).enumeration(
                            Gender::Table,
                            [Gender::Male, Gender::Female, Gender::Other],
                        ),
                    )
                    .col(ColumnDef::new(UserProfile::Image).string())
                    .col(ColumnDef::new(UserProfile::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_profile_user_id")
                            .from(UserProfile::Table, UserProfile::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(UserProfile::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(UserProfile::UpdatedAt)
                            .date_time()
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
            .drop_table(Table::drop().table(UserProfile::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Gender::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum UserProfile {
    Table,
    Id,
    FirstName,
    LastName,
    Phone,
    DateOfBirth,
    Gender,
    Image,
    UserId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Gender {
    Table,
    Male,
    Female,
    Other,
}
