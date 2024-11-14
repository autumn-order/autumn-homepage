use dioxus::prelude::*;

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
