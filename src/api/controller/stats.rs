use dioxus::prelude::*;

use crate::model::stats::StatsDto;

#[server]
pub async fn get_stats() -> Result<Vec<StatsDto>, ServerFnError> {
    use crate::api::data::stats::StatsRepository;
    use sea_orm::DatabaseConnection;

    let db: axum::Extension<DatabaseConnection> = extract().await?;

    let stats_repository = StatsRepository::new(&db);

    let stats = stats_repository.get(vec![], 0, 60).await?;

    let stats_dtos: Vec<StatsDto> = stats.into_iter().map(|s| s.into()).collect();

    Ok(stats_dtos)
}
