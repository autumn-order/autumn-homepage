use autumn_homepage::api::constant::APP_VERSION_INFO;
use autumn_homepage::api::data::stats::StatsRepository;
use autumn_homepage::api::service::stats::update_corporation_stats;

use dotenv::dotenv;
use mockito::{Mock, ServerGuard};
use sea_orm::{ConnectionTrait, Database, DbBackend, Schema};

#[tokio::test]
async fn test_update_corporation_stats() {
    fn create_mock(mock_server: &mut ServerGuard, endpoint: &str, body: &str) -> Mock {
        mock_server
            .mock("GET", endpoint)
            .expect(1)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    dotenv().ok();

    let db = Database::connect("sqlite::memory:").await.unwrap();
    let schema = Schema::new(DbBackend::Sqlite);

    db.execute(
        db.get_database_backend()
            .build(&schema.create_table_from_entity(entity::prelude::Stats)),
    )
    .await
    .unwrap();

    let mut mock_server = mockito::Server::new_async().await;
    let mock_server_url = mock_server.url();

    let mock1_body = r#"
    {
        "alliance_id": 99012770,
        "ceo_id": 2114794365,
        "creator_id": 2114794365,
        "date_founded": "2024-10-07T21:43:09Z",
        "description": "",
        "home_station_id": 60003760,
        "member_count": 21,
        "name": "The Order of Autumn",
        "shares": 1000,
        "tax_rate": 0,
        "ticker": "F4LL.",
        "url": "http://",
        "war_eligible": true
    }
    "#;

    let mock2_body = r#"
    {
        "ceo_id": 2117034825,
        "creator_id": 2114794365,
        "date_founded": "2024-09-25T06:16:01Z",
        "description": "",
        "home_station_id": 60003760,
        "member_count": 1,
        "name": "Autumn Highsec Division",
        "shares": 1000,
        "tax_rate": 0,
        "ticker": "ATMNH",
        "url": "http://"
    }
    "#;

    let mock1 = create_mock(
        &mut mock_server,
        "/corporations/98785281/?datasource=tranquility",
        mock1_body,
    );

    let mock2 = create_mock(
        &mut mock_server,
        "/corporations/98784256/?datasource=tranquility",
        mock2_body,
    );

    let mock3 = mock_server
        .mock("GET", "/corporations/99999999/?datasource=tranquility")
        .expect(2)
        .with_status(404)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": "Corporation not found"}"#)
        .create();

    let application_email =
        std::env::var("APPLICATION_EMAIL").expect("APPLICATION_EMAIL is not set in .env");
    let user_agent = format!("{} ({})", APP_VERSION_INFO, application_email);

    let mut esi_client = eve_esi::Client::new(&user_agent);

    esi_client.esi_url = mock_server_url.to_string();

    // Test error handling for corporation not found when using id 99999999
    // update_corporation_stats is supposed to skip any corporation request that returns a 404
    let corporation_ids = &[98785281, 98784256, 99999999];

    // Call twice to ensure only 1 entry per corporation is created per day
    update_corporation_stats(&db, &esi_client, corporation_ids).await;
    update_corporation_stats(&db, &esi_client, corporation_ids).await;

    let stats_repository = StatsRepository::new(&db);

    let entries = stats_repository.get(vec![], 0, 2).await.unwrap();

    println!("{:#?}", entries);

    mock1.assert();
    mock2.assert();
    mock3.assert();
    assert!(entries.len() == 2);
}
