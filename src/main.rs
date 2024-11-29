#![allow(non_snake_case)]

mod api;
mod model;
mod web;

use dioxus::prelude::*;

use web::App;

fn main() {
    #[cfg(feature = "web")]
    launch(App);

    #[cfg(feature = "server")]
    {
        use api::update::schedule_tasks;
        use axum::routing::*;
        use axum::Extension;
        use dioxus_logger::tracing::{info, Level};
        use migration::{Migrator, MigratorTrait};
        use sea_orm::{ConnectOptions, Database};
        use tokio_cron_scheduler::JobScheduler;

        dotenv::dotenv().ok();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let database_url =
                    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

                let mut opt = ConnectOptions::new(database_url);
                opt.sqlx_logging(false);

                let db = Database::connect(opt)
                    .await
                    .expect("Failed to connect to the database");

                Migrator::up(&db, None)
                    .await
                    .expect("Failed to run migrations");

                let sched = JobScheduler::new()
                    .await
                    .expect("Failed to create JobScheduler");

                dioxus_logger::init(Level::INFO).expect("failed to init logger");
                info!("Starting server");

                schedule_tasks(&sched, &db).await;

                sched.start().await.expect("Failed to start scheduler");

                let router = Router::new()
                    .serve_dioxus_application(
                        ServeConfigBuilder::new().index_html(
                            r#"
                                <!DOCTYPE html>
                                <html>
                                <head>
                                    <title>Autumn</title>
                                    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                                    <link rel="stylesheet" href="/assets/tailwind.css" />
                                    <script defer src="https://analytics.autumn-order.com/script.js" data-website-id="ce8c52d1-65eb-4493-952f-2732eae3f11b"></script>
                                </head>
                                <body>
                                    <div id="main"></div>
                                </body>
                                </html>
                            "#.into(),
                        ),
                        App,
                    )
                    .layer(Extension(db));

                let router = router.into_make_service();
                let address = dioxus_cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(address).await.unwrap();
                axum::serve(listener, router).await.unwrap();
            });
    }
}
