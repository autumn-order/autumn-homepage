use std::env;

use sea_orm::{ColumnTrait, Database, DatabaseConnection};

use crate::api::{
    constant::{USER_AGENT, ZKILL_URL},
    data::stats::StatsRepository,
    service::zkill::get_zkill_corporation_stats,
};

const fn get_autumn_corporation_ids() -> &'static [i32] {
    &[98785281, 98784256]
}

pub async fn get_corporation_stats(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    const AUTUMN_CORPORATION_IDS: &[i32] = get_autumn_corporation_ids();

    let client: reqwest::Client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    let stats_repository = StatsRepository::new(&db);

    for corporation_id in AUTUMN_CORPORATION_IDS {
        let filters = vec![entity::stats::Column::CorporationId.eq(*corporation_id)];
        let recent_entries = stats_repository.get(filters, 0, 1).await?;

        if !recent_entries.is_empty() {
            let recent_entry = &recent_entries[0];
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(recent_entry.date);

            if duration.num_hours() < 24 {
                continue;
            }
        }

        let result = get_zkill_corporation_stats(&client, ZKILL_URL, *corporation_id).await;

        let stats = match result {
            Ok(stats) => stats,
            Err(_) => continue,
        };

        stats_repository
            .create(
                *corporation_id,
                stats.info.member_count,
                stats.ships_destroyed,
            )
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

    use super::*;

    #[tokio::test]
    async fn test_get_corporation_stats() {
        let db = Database::connect("sqlite::memory:").await.unwrap();
        let schema = Schema::new(DbBackend::Sqlite);

        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(entity::prelude::Stats)),
        )
        .await
        .unwrap();

        let result = get_corporation_stats(&db).await;

        let stats_repository = StatsRepository::new(&db);

        let expected_length = get_autumn_corporation_ids().len();
        let entries = stats_repository
            .get(vec![], 0, expected_length.try_into().unwrap())
            .await
            .unwrap();

        assert!(result.is_ok());
        assert!(entries.len() == expected_length);
    }
}
