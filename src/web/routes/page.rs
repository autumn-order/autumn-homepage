use dioxus::prelude::*;

use crate::web::components::{home::sections::Hero, Footer, Header};

#[component]
pub fn Home() -> Element {
    rsx! {
        Header {}
        Hero {}
        Footer {}
    }
}
