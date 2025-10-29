use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "species_of_plants",
            &[
            
            ("id", ColType::PkAuto),
            
            ("common_name", ColType::String),
            ("scientific_name", ColType::StringNull),
            ("family_name", ColType::StringNull),
            ("care_instructions", ColType::TextNull),
            ("origin", ColType::StringNull),
            ("photo_url", ColType::JsonBinaryNull),
            ],
            &[
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "species_of_plants").await
    }
}
