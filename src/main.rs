#![allow(non_snake_case)]

#[cfg(feature = "server")]
mod api;

mod web;

use dioxus::prelude::*;
use dioxus_logger::tracing;

use web::Route;

fn main() {
    #[cfg(feature = "server")]
    {
        dotenv::dotenv().ok();
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
