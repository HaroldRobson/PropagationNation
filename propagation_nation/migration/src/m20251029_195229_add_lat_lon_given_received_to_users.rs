use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        add_column(m, "users", "lat", ColType::FloatNull).await?;
        add_column(m, "users", "lon", ColType::FloatNull).await?;
        add_column(m, "users", "given", ColType::IntegerNull).await?;
        add_column(m, "users", "received", ColType::IntegerNull).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        remove_column(m, "users", "lat").await?;
        remove_column(m, "users", "lon").await?;
        remove_column(m, "users", "given").await?;
        remove_column(m, "users", "received").await?;
        Ok(())
    }
}
