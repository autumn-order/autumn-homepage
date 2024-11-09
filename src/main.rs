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
        use sea_orm::Database;
        use std::sync::Arc;
        use tokio_cron_scheduler::JobScheduler;

        dotenv::dotenv().ok();

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let database_url =
                    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

                let db = Arc::new(
                    Database::connect(database_url)
                        .await
                        .expect("Failed to connect to the database"),
                );

                let sched = JobScheduler::new()
                    .await
                    .expect("Failed to create JobScheduler");

                let db_clone = Arc::clone(&db);
                schedule_tasks(&sched, db_clone).await;

                sched.start().await.expect("Failed to start scheduler");

                // This needs a .layer(Extension(db)) when it is determined how to access it from a Dioxus server function
                let app = Router::new()
                    .serve_dioxus_application(ServeConfig::builder().build(), || {
                        VirtualDom::new(App)
                    })
                    .await;

                let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
