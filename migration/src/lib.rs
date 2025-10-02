pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20220101_000002_seed_characters;
mod m20250102_000001_schema_update;
mod m20250102_000002_update_character_names;
mod m20250102_000003_add_gsp_to_sessions;
mod m20250102_000004_add_ffa_character;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20220101_000002_seed_characters::Migration),
            Box::new(m20250102_000001_schema_update::Migration),
            Box::new(m20250102_000002_update_character_names::Migration),
            Box::new(m20250102_000003_add_gsp_to_sessions::Migration),
            Box::new(m20250102_000004_add_ffa_character::Migration),
        ]
    }
}
