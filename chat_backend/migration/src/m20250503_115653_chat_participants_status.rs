use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                   .table(ChatParticipants::Table)
                   .add_column(
                        ColumnDef::new(ChatParticipants::Status)
                            .string()
                            .not_null()
                            .default(Expr::value("pending"))
                   )
                   .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ChatParticipants::Table)
                    .drop_column(ChatParticipants::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
#[allow(dead_code)]
enum ChatParticipants {
    Table,
    Id,
    ChatId,
    UserId,
    Status,
}