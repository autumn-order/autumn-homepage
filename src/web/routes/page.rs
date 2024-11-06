use dioxus::prelude::*;
use dioxus_helmet::Helmet;

use crate::web::components::{home::sections::Hero, Footer, Header};

#[component]
pub fn Home() -> Element {
    rsx! {
        Helmet {
            title { "Autumn" }
            meta {
                name: "description",
                content: "EVE is complicated, Autumn makes it easy. There are many twists and turns in the beginning of an EVE journey, why waste time learning the hard way when you can learn the right way?"
            }
        }
        Header {}
        Hero {}
        Footer {}
    }
}
