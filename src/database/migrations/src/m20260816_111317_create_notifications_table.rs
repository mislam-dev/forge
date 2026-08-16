use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20260816_111317_create_notifications_table"
    }
}

const FK_NOTIFICATIONS_USER_ID: &str = "fk_notifications_user_id";

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Notifications::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notifications::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(ColumnDef::new(Notifications::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_NOTIFICATIONS_USER_ID)
                            .from(Notifications::Table, Notifications::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Notifications::Type)
                            .string_len(100)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Notifications::Title)
                            .string_len(255)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notifications::Message).text().not_null())
                    .col(ColumnDef::new(Notifications::ReferenceId).uuid().null())
                    .col(ColumnDef::new(Notifications::ReferenceType).string().null())
                    .col(
                        ColumnDef::new(Notifications::IsRead)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(Notifications::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Composite index for the primary access pattern: "unread notifications
        // for user X, newest first". Column order matters here — user_id first
        // (equality filter), then is_read (equality filter), then created_at DESC
        // (sort) — this ordering lets Postgres use the index for both the WHERE
        // and the ORDER BY in one pass.
        manager
            .create_index(
                Index::create()
                    .name("idx_notifications_user_unread_created")
                    .table(Notifications::Table)
                    .col(Notifications::UserId)
                    .col(Notifications::IsRead)
                    .col((Notifications::CreatedAt, IndexOrder::Desc))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notifications::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Notifications {
    Table,
    Id,
    UserId,
    Type,
    Title,
    Message,
    ReferenceId,
    ReferenceType,
    IsRead,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
