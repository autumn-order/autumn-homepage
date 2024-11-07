use chrono::Utc;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Stats::Table)
                    .if_not_exists()
                    .col(pk_auto(Stats::Id))
                    .col(integer(Stats::CorporationId).not_null())
                    .col(integer(Stats::MemberCount).not_null().default(0))
                    .col(integer(Stats::ShipsDestroyed).not_null().default(0))
                    .col(timestamp(Stats::Date).not_null().default(Utc::now()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Stats::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Stats {
    Table,
    Id,
    CorporationId,
    MemberCount,
    ShipsDestroyed,
    Date,
}
