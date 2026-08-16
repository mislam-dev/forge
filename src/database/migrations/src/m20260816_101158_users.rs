use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_query::extension::postgres::Type;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_101158_users"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(UserStatus::Table)
                    .values([
                        UserStatus::Active,
                        UserStatus::Unverified,
                        UserStatus::Disabled,
                        UserStatus::Suspended,
                        UserStatus::Inactive,
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .col(
                        ColumnDef::new(Users::EmailVerified)
                            .boolean()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Users::Status)
                            .enumeration(
                                UserStatus::Table,
                                [
                                    UserStatus::Active,
                                    UserStatus::Unverified,
                                    UserStatus::Disabled,
                                    UserStatus::Suspended,
                                    UserStatus::Inactive,
                                ],
                            )
                            .default("unverified"),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
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
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(UserStatus::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
    PasswordHash,
    EmailVerified,
    Status,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum UserStatus {
    #[sea_orm(iden = "user_status")]
    Table,
    Active,
    Unverified,
    Disabled,
    Suspended,
    Inactive,
}
