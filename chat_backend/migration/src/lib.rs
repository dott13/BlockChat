pub use sea_orm_migration::prelude::*;

mod m20241213_220758_init_schema;
mod m20250313_212955_create_chats_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241213_220758_init_schema::Migration),
            Box::new(m20250313_212955_create_chats_table::Migration),
        ]
    }
}
