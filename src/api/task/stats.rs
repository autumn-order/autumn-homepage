use sea_orm::DatabaseConnection;

use crate::api::{constant::APP_VERSION_INFO, service::stats::update_corporation_stats};

const fn get_autumn_corporation_ids() -> &'static [i32] {
    &[98785281, 98784256]
}

pub async fn task_update_corporation_stats(db: &DatabaseConnection) {
    let application_email =
        std::env::var("APPLICATION_EMAIL").expect("APPLICATION_EMAIL is not set in .env");

    let user_agent = format!("{} ({})", APP_VERSION_INFO, application_email);
    let esi_client = eve_esi::Client::new(&user_agent);

    const CORPORATION_IDS: &[i32] = get_autumn_corporation_ids();

    update_corporation_stats(db, &esi_client, CORPORATION_IDS).await;
}
