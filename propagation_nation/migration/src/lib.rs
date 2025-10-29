#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20251029_194315_species_of_plants;
mod m20251029_194711_plants;
mod m20251029_194834_chats;
mod m20251029_195229_add_lat_lon_given_received_to_users;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20251029_194315_species_of_plants::Migration),
            Box::new(m20251029_194711_plants::Migration),
            Box::new(m20251029_194834_chats::Migration),
            Box::new(m20251029_195229_add_lat_lon_given_received_to_users::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}