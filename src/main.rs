#![allow(non_snake_case)]

mod api;
mod web;

use dioxus::prelude::*;
use dioxus_logger::tracing;

use web::Route;

fn main() {
    #[cfg(feature = "server")]
    {
        use api::update::schedule_tasks;
        use std::thread;

        dotenv::dotenv().ok();

        thread::spawn(|| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                schedule_tasks().await.unwrap();
            });
        });
    }

    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
