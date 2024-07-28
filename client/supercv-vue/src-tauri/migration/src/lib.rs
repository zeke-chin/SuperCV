pub use sea_orm_migration::prelude::*;

mod m20240714_065956_create_clipboard_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240714_065956_create_clipboard_table::Migration),
        ]
    }
}
