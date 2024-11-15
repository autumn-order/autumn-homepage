use chrono::{Duration, Utc};
use log::warn;
use sea_orm::{ColumnTrait, DatabaseConnection};

use crate::api::data::stats::StatsRepository;

pub async fn update_corporation_stats(
    db: &DatabaseConnection,
    esi_client: &eve_esi::Client,
    corporation_ids: &[i32],
) {
    let stats_repository = StatsRepository::new(db);

    for corporation_id in corporation_ids {
        let filters = vec![entity::stats::Column::CorporationId.eq(*corporation_id)];

        let recent_entries = match stats_repository.get(filters, 0, 1).await {
            Ok(entries) => entries,
            Err(error) => {
                warn!(
                    "Failed to retrieve entries for corporation {} from stats table in database: {}",
                    corporation_id, error
                );

                continue;
            }
        };

        if !recent_entries.is_empty() {
            let recent_entry = &recent_entries[0];

            let now = Utc::now();
            if now - recent_entry.date < Duration::hours(24) {
                continue;
            }
        }

        let corporation = match esi_client.get_corporation(*corporation_id).await {
            Ok(corporation) => corporation,
            Err(error) => {
                warn!(
                    "Failed to retrieve corporation {} from ESI: {}",
                    corporation_id, error
                );

                continue;
            }
        };

        match stats_repository
            .create(*corporation_id, corporation.member_count)
            .await
        {
            Ok(_) => (),
            Err(error) => {
                warn!(
                    "Failed to save corporation {} to database: {}",
                    corporation_id, error
                );

                continue;
            }
        };
    }
}
