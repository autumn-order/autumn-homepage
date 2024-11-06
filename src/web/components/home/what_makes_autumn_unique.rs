use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::fa_solid_icons::{FaAtom, FaChartLine},
    Icon,
};

#[component]
pub fn WhatMakesAutumnUnique() -> Element {
    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "flex flex-col gap-6 py-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl pb-4",
                        "What Sets Autumn Apart?"
                    }
                    ul { class: "flex justify-evenly gap-4 md:gap-0 flex-wrap",
                        li { class: "w-full md:w-1/2 md:pr-2",
                            div { class: "card card-compact shadow md:h-48 lg:h-32",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        Icon { icon: FaAtom, width: 24, height: 24 }
                                        span { "IT Infrastructure From the Ground Up" }
                                    }
                                    p {
                                        "In a sea of many alliances & corporations few have the ability to design their own IT infrastructure from the ground up. A distinct advantage when it comes to achieving our goal of being highly organized and highly accessible to even the newest of players."
                                    }
                                }
                            }
                        }
                        li { class: "w-full md:w-1/2 md:pl-2",
                            div { class: "card card-compact shadow md:h-48 lg:h-32",
                                div { class: "card-body",
                                    h2 { class: "card-title",
                                        Icon { icon: FaChartLine, width: 24, height: 24 }
                                        span { "Not Just Education But Continued Growth" }
                                    }
                                    p {
                                        "Not only do we aim to help you start those first steps in EVE Online, we aim to take you all the way towards your end goal. At Autumn investment in our member's growth comes first and foremost."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
