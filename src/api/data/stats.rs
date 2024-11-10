use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, PaginatorTrait, QueryOrder};
use sea_orm::{EntityTrait, QueryFilter};

use entity::prelude::Stats;
use entity::stats::ActiveModel;
use entity::stats::Model;

pub struct StatsRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> StatsRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        corporation_id: i32,
        member_count: i32,
    ) -> Result<Model, sea_orm::DbErr> {
        ActiveModel {
            corporation_id: ActiveValue::set(corporation_id),
            member_count: ActiveValue::set(member_count),
            date: ActiveValue::set(chrono::Utc::now()),
            ..Default::default()
        }
        .insert(self.db)
        .await
    }

    pub async fn get(
        &self,
        filters: Vec<migration::SimpleExpr>,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<Model>, sea_orm::DbErr> {
        let mut query = Stats::find();

        for filter in filters {
            query = query.filter(filter);
        }

        query
            .order_by_desc(entity::stats::Column::Date)
            .paginate(self.db, page_size)
            .fetch_page(page)
            .await
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::{ColumnTrait, ConnectionTrait, Database, DbBackend, Schema};

    use super::*;

    async fn setup_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let schema = Schema::new(DbBackend::Sqlite);

        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(entity::prelude::Stats)),
        )
        .await
        .unwrap();

        db
    }

    #[tokio::test]
    async fn create_stats_entry() {
        let db = setup_db().await;

        let repository = StatsRepository::new(&db);

        let corporation_id = 12345;
        let member_count = 189;

        let result = repository
            .create(corporation_id, member_count)
            .await
            .unwrap();

        assert_eq!(result.corporation_id, corporation_id);
        assert_eq!(result.member_count, member_count);
    }

    #[tokio::test]
    async fn get_stats_entry() {
        let db = setup_db().await;

        let repository = StatsRepository::new(&db);

        let corporation_id = 12345;
        let member_count = 189;

        let _ = repository
            .create(corporation_id, member_count)
            .await
            .unwrap();

        let filters = vec![entity::stats::Column::CorporationId.eq(corporation_id)];

        let result = repository.get(filters, 0, 1).await.unwrap();

        assert!(!result.is_empty());
        assert_eq!(result[0].corporation_id, corporation_id);
        assert_eq!(result[0].member_count, member_count);
    }
}
