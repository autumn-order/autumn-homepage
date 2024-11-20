#![allow(non_snake_case)]

mod api;
mod model;
mod web;

use dioxus::prelude::*;

use web::Route;

fn main() {
    #[cfg(feature = "web")]
    launch(App);

    #[cfg(feature = "server")]
    {
        use api::update::schedule_tasks;
        use axum::routing::*;
        use axum::Extension;
        use migration::{Migrator, MigratorTrait};
        use sea_orm::Database;
        use tokio_cron_scheduler::JobScheduler;

        dotenv::dotenv().ok();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let database_url =
                    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

                let db = Database::connect(database_url)
                    .await
                    .expect("Failed to connect to the database");

                Migrator::up(&db, None)
                    .await
                    .expect("Failed to run migrations");

                let sched = JobScheduler::new()
                    .await
                    .expect("Failed to create JobScheduler");

                schedule_tasks(&sched, &db).await;

                sched.start().await.expect("Failed to start scheduler");

                let router = Router::new()
                    .serve_dioxus_application(ServeConfigBuilder::default(), App)
                    .layer(Extension(db));

                let router = router.into_make_service();
                let address = dioxus_cli_config::fullstack_address_or_localhost();
                let listener = tokio::net::TcpListener::bind(address).await.unwrap();
                axum::serve(listener, router).await.unwrap();
            });
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
