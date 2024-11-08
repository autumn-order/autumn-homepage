use sea_orm::Database;

use crate::api::{constant::USER_AGENT, service::stats::update_corporation_stats};

const fn get_autumn_corporation_ids() -> &'static [i32] {
    &[98785281, 98784256]
}

pub async fn task_update_corporation_stats() {
    let application_email =
        std::env::var("APPLICATION_EMAIL").expect("APPLICATION_EMAIL is not set in .env");
    let database_url = std::env::var("DATABASE_URL").expect("APPLICATION_EMAIL is not set in .env");

    let db = Database::connect(database_url).await.unwrap();
    let user_agent = format!("{} ({})", USER_AGENT, application_email);
    let reqwest_client = reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .unwrap();
    let esi_client = eve_esi::Client::new(reqwest_client);

    const CORPORATION_IDS: &[i32] = get_autumn_corporation_ids();

    update_corporation_stats(&db, &esi_client, CORPORATION_IDS).await;
}
