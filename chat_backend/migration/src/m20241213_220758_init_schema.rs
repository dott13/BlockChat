use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string().not_null().unique_key())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Blocks::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Blocks::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Blocks::Name).string().not_null().unique_key())
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
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::FirstName).string().not_null())
                    .col(ColumnDef::new(Users::LastName).string().not_null())
                    .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(ColumnDef::new(Users::Avatar).binary())
                    .col(ColumnDef::new(Users::RoleId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(ColumnDef::new(Users::BlockId).integer().null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Users::Table, Users::BlockId)
                            .to(Blocks::Table, Blocks::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Messages::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Messages::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Messages::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Messages::Table, Messages::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Messages::BlockId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Messages::Table, Messages::BlockId)
                            .to(Blocks::Table, Blocks::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Messages::Content).string().not_null())
                    .col(ColumnDef::new(Messages::Metadata).json_binary())
                    .col(
                        ColumnDef::new(Messages::Timestamp)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Messages::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Users::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Blocks::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Roles::Table).to_owned()).await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Roles {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Blocks {
    Table,
    Id,
    Name,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
    FirstName,
    LastName,
    Username,
    Password,
    Avatar,
    RoleId,
    BlockId,
    CreatedAt,
}

#[derive(Iden)]
enum Messages {
    Table,
    Id,
    UserId,
    BlockId,
    Content,
    Metadata,
    Timestamp,
}
