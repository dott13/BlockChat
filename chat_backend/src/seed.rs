use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};
use crate::entities::roles;

pub async fn seed_roles(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Check if the roles table is empty
    let role_count = roles::Entity::find().count(db).await?;

    if role_count == 0 {
        // Roles to seed
        let roles_to_add = vec![
            roles::ActiveModel {
                name: Set(String::from("chat_admin")),
                ..Default::default()
            },
            roles::ActiveModel {
                name: Set(String::from("admin")),
                ..Default::default()
            },
            roles::ActiveModel {
                name: Set(String::from("user")),
                ..Default::default()
            },
        ];

        // Insert roles into the database
        for role in roles_to_add {
            roles::Entity::insert(role).exec(db).await?;
        }
        println!("Roles seeded successfully.");
    } else {
        println!("Roles already exist. Skipping seeding.");
    }

    Ok(())
}
