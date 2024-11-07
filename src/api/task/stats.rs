use crate::api::{
    constant::{USER_AGENT, ZKILL_URL},
    service::zkill::get_zkill_corporation_stats,
};

const fn get_autumn_corporation_ids() -> &'static [i32] {
    &[98785281, 98784256]
}

pub async fn get_corporation_stats() {
    const AUTUMN_CORPORATION_IDS: &[i32] = get_autumn_corporation_ids();
    let client: reqwest::Client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .unwrap();

    // init stats repository

    for corporation_id in AUTUMN_CORPORATION_IDS {
        let result = get_zkill_corporation_stats(&client, ZKILL_URL, *corporation_id).await;

        // save to stats repository
    }
}
