use sea_orm_migration::prelude::*;
use sea_query::Expr;
use sea_orm_migration::prelude::extension::postgres::Extension;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS pgcrypto")
        .await?;
        // 1. Alter Users: Drop the deprecated BlockId column.
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::BlockId)
                    .to_owned(),
            )
            .await?;
            
        // 2a. Alter Messages: Drop the foreign key on BlockId.
        manager
        .alter_table(
            Table::alter()
                .table(Messages::Table)
                .drop_foreign_key(Alias::new("messages_block_id_fkey"))
                .to_owned(),
        )
        .await?;

        // 2b. Since there's no data, drop the old column and add a new UUID column
        manager
        .alter_table(
            Table::alter()
                .table(Messages::Table)
                .drop_column(Messages::BlockId)
                .to_owned(),
        )
        .await?;

        // 2c. Add the new ChatId column as UUID
        manager
        .alter_table(
            Table::alter()
                .table(Messages::Table)
                .add_column(
                    ColumnDef::new(Messages::ChatId)
                        .uuid()
                        .not_null()
                        .default(Expr::cust("gen_random_uuid()"))
                )
                .to_owned(),
        )
        .await?;
            
        // 3. Create the Chats table (replacing Blocks).
        let mut fk_chats_author = {
            let mut fk = ForeignKey::create();
            fk.from(Chats::Table, Chats::AuthorId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade);
            fk
        };
        manager
            .create_table(
                Table::create()
                    .table(Chats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chats::Id)
                        .uuid()                                         // ← was `.integer()`
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("gen_random_uuid()")),      // ← instead of auto_increment
                        )
                    .col(
                        ColumnDef::new(Chats::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Chats::AuthorId).integer().not_null())
                    .foreign_key(&mut fk_chats_author)
                    .col(ColumnDef::new(Chats::Image).binary().null())
                    .col(
                        ColumnDef::new(Chats::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
            
        // 4. Create the ChatParticipants table.
        let mut fk_cp_chat = {
            let mut fk = ForeignKey::create();
            fk.from(ChatParticipants::Table, ChatParticipants::ChatId)
                .to(Chats::Table, Chats::Id)
                .on_delete(ForeignKeyAction::Cascade);
            fk
        };
        let mut fk_cp_user = {
            let mut fk = ForeignKey::create();
            fk.from(ChatParticipants::Table, ChatParticipants::UserId)
                .to(Users::Table, Users::Id)
                .on_delete(ForeignKeyAction::Cascade);
            fk
        };
        manager
            .create_table(
                Table::create()
                    .table(ChatParticipants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ChatParticipants::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ChatParticipants::ChatId).uuid().not_null())
                    .col(ColumnDef::new(ChatParticipants::UserId).integer().not_null())
                    .foreign_key(&mut fk_cp_chat)
                    .foreign_key(&mut fk_cp_user)
                    .to_owned(),
            )
            .await?;
            
        // 5. Alter Messages: Add a new foreign key on ChatId referencing Chats.
        let fk_messages_chat = {
            let mut fk = ForeignKey::create();
            fk.from(Messages::Table, Messages::ChatId)
                .to(Chats::Table, Chats::Id)
                .on_delete(ForeignKeyAction::Cascade);
            fk.get_foreign_key().clone()
        };
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_foreign_key(&fk_messages_chat)
                    .to_owned(),
            )
            .await?;
            
        // 6. Drop the now-deprecated Blocks table.
        manager
            .drop_table(Table::drop().table(Blocks::Table).to_owned())
            .await?;
            
        Ok(())
    }
    
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. Recreate the Blocks table.
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
                    .col(
                        ColumnDef::new(Blocks::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;
            
        // 2a. Alter Messages: Drop the foreign key on ChatId.
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .drop_foreign_key(Alias::new("messages_chat_id_fkey"))
                    .to_owned(),
            )
            .await?;
            
        // 2b. Alter Messages: Rename ChatId back to BlockId.
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .rename_column(Messages::ChatId, Messages::BlockId)
                    .to_owned(),
            )
            .await?;
            
        // 3. Drop the ChatParticipants table.
        manager
            .drop_table(Table::drop().table(ChatParticipants::Table).to_owned())
            .await?;
            
        // 4. Drop the Chats table.
        manager
            .drop_table(Table::drop().table(Chats::Table).to_owned())
            .await?;
            
        // 5. Alter Users: add back the BlockId column.
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::BlockId).integer().null())
                    .to_owned(),
            )
            .await?;
            
        // 6. Add foreign key on Users.BlockId referencing Blocks.
        let fk_users_block = {
            let mut fk = ForeignKey::create();
            fk.from(Users::Table, Users::BlockId)
                .to(Blocks::Table, Blocks::Id)
                .on_delete(ForeignKeyAction::SetNull);
            fk.get_foreign_key().clone()
        };
        manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_foreign_key(&fk_users_block)
                    .to_owned(),
            )
            .await?;
            
        // 7. Add foreign key on Messages.BlockId referencing Blocks.
        let fk_messages_block = {
            let mut fk = ForeignKey::create();
            fk.from(Messages::Table, Messages::BlockId)
                .to(Blocks::Table, Blocks::Id)
                .on_delete(ForeignKeyAction::Cascade);
            fk.get_foreign_key().clone()
        };
        manager
            .alter_table(
                Table::alter()
                    .table(Messages::Table)
                    .add_foreign_key(&fk_messages_block)
                    .to_owned(),
            )
            .await?;
            
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
enum Chats {
    Table,
    Id,
    Name,
    AuthorId,
    Image,
    CreatedAt,
}

#[derive(Iden)]
enum ChatParticipants {
    Table,
    Id,
    ChatId,
    UserId,
}

#[derive(Iden)]
enum Messages {
    Table,
    Id,
    UserId,
    // Both the old and new column names are defined.
    BlockId,
    ChatId,
    Content,
    Metadata,
    Timestamp,
}
