use std::env;

use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2};
use argon2::PasswordHasher;
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};
use crate::entities::{roles, users};

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

pub async fn seed_users(db: &DatabaseConnection) -> Result<(), DbErr> {
    let user_count = users::Entity::find().count(db).await?;
    if user_count == 0 {
        dotenvy::dotenv().ok();

        let admin_pw = env::var("ADMIN_PASSWORD").unwrap_or_else(|_| "admin123".into());
        let user_pw  = env::var("USER_PASSWORD").unwrap_or_else(|_| "user123".into());

        fn hash_pw(pw: &str) -> String {
            let mut rng = OsRng;
            let salt = SaltString::generate(&mut rng);

            Argon2::default()
                .hash_password(pw.as_bytes(), &salt)
                .unwrap()
                .to_string()
        }

        let admin = users::ActiveModel {
            first_name: Set("Super".into()),
            last_name: Set("Admin".into()),
            username: Set("admin".into()),
            password: Set(hash_pw(&admin_pw)),
            role_id: Set(Some(3)),
            ..Default::default()
        };

        let user = users::ActiveModel {
            first_name: Set("Regular".into()),
            last_name: Set("User".into()),
            username: Set("user".into()),
            password: Set(hash_pw(&user_pw)),
            role_id: Set(Some(3)),
            ..Default::default()
        };

        users::Entity::insert(admin).exec(db).await?;
        users::Entity::insert(user).exec(db).await?;
        println!("Seeded admin/user from env passwords.");
    }
    Ok(())
}