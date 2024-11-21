use dioxus::prelude::*;
use dioxus_document::{Meta, Title};

use crate::web::components::{home::sections::Hero, Footer, Header};

#[component]
pub fn Home() -> Element {
    rsx! {
        Title { "Autumn" }
        Meta {
            name: "description",
            content: "The Order of Autumn is an EVE Online corporation part of Black Rose alliance & Phoenix Coalition. We are real life first & new player focused with an emphasis on organization, community, and high quality IT infrastructure."
        }
        Header {}
        Hero {}
        Footer {}
    }
}
