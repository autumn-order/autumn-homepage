use sea_orm::{ColumnTrait, DatabaseConnection};

use crate::api::data::stats::StatsRepository;

pub async fn get_corporation_stats(
    db: &DatabaseConnection,
    esi_client: &eve_esi::Client,
    corporation_ids: &[i32],
) -> Result<(), sea_orm::DbErr> {
    let stats_repository = StatsRepository::new(db);

    for corporation_id in corporation_ids {
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

        let corporation = match esi_client.get_corporation(*corporation_id).await {
            Ok(corporation) => corporation,
            Err(_) => continue,
        };

        stats_repository
            .create(*corporation_id, corporation.member_count)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

    use crate::api::constant::USER_AGENT;

    use super::*;

    #[tokio::test]
    async fn test_get_corporation_stats() {
        dotenv().ok();

        let db = Database::connect("sqlite::memory:").await.unwrap();
        let schema = Schema::new(DbBackend::Sqlite);

        db.execute(
            db.get_database_backend()
                .build(&schema.create_table_from_entity(entity::prelude::Stats)),
        )
        .await
        .unwrap();

        let application_email =
            std::env::var("APPLICATION_EMAIL").expect("APPLICATION_EMAIL is not set in .env");
        let user_agent = format!("{} ({})", USER_AGENT, application_email);
        let reqwest_client = reqwest::Client::builder()
            .user_agent(user_agent)
            .build()
            .unwrap();
        let esi_client = eve_esi::Client::new(reqwest_client);

        let corporation_ids = &[98785281, 98784256];
        let result = get_corporation_stats(&db, &esi_client, corporation_ids).await;

        let stats_repository = StatsRepository::new(&db);

        let expected_length = corporation_ids.len();
        let entries = stats_repository
            .get(vec![], 0, expected_length.try_into().unwrap())
            .await
            .unwrap();

        assert!(result.is_ok());
        assert!(entries.len() == expected_length);
    }
}
