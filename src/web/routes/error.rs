use dioxus::prelude::*;
use dioxus_document::{Meta, Title};

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx! {
        Title { "404 Not Found" }
        Meta {
            name: "description",
            content: "The page you are looking for does not exist."
        }
        section { class: "min-h-screen w-screen flex flex-col gap-2 items-center justify-center py-4",
            h1 { class: "text-3xl md:text-4xl", "404 Not Found" }
            p { class:"text-xl", "The page you are looking for does not exist." }
        }
    }
}
