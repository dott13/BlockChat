use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "chat_participants")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub chat_id: Uuid,
    pub user_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chats::Entity",
        from = "Column::ChatId",
        to = "super::chats::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Chat,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<crate::entities::chats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chat.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
