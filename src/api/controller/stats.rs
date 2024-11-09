use dioxus::prelude::*;

use crate::model::stats::StatsDto;

#[server]
pub async fn get_stats() -> Result<Vec<StatsDto>, ServerFnError> {
    use crate::api::data::stats::StatsRepository;
    use sea_orm::Database;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to the database");

    let stats_repository = StatsRepository::new(&db);

    let stats = stats_repository.get(vec![], 0, 60).await?;

    let stats_dtos: Vec<StatsDto> = stats.into_iter().map(|s| s.into()).collect();

    Ok(stats_dtos)
}
