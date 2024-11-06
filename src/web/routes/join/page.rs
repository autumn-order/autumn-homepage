use dioxus::prelude::*;
use dioxus_helmet::Helmet;

use crate::web::components::join::JoinHeader;

use crate::web::components::join::{
    section::{
        CallToAction, Endgame, FrequentlyAskedQuestions, Hero, LearningCurve, WhatMakesAutumnUnique,
    },
    JoinFooter,
};

#[component]
pub fn JoinAutumn() -> Element {
    rsx! {
        Helmet {
            title { "Join Autumn" }
            meta {
                name: "description",
                content: "EVE is complicated, Autumn makes it easy. There are many twists and turns in the beginning of an EVE journey, why waste time learning the hard way when you can learn the right way?"
            }
        }
        JoinHeader {}
        Hero {}
        Endgame {}
        LearningCurve {}
        WhatMakesAutumnUnique {}
        CallToAction {}
        FrequentlyAskedQuestions {}
        JoinFooter {}
    }
}
