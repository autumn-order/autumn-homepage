#![allow(non_snake_case)]

pub mod api;
pub mod web;

use dotenv::dotenv;

use dioxus::prelude::*;
use dioxus_logger::tracing;

use web::Route;

fn main() {
    dotenv().ok();

    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
