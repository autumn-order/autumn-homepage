use dioxus::prelude::*;
use manganis::Asset;

use crate::web::constant::FEATURED_VIDEO;

#[component]
pub fn Endgame() -> Element {
    const FLEET_COMMANDER_IMAGE: Asset = manganis::asset!("/assets/images/join/monitor.png");
    const CORPORATION_LEADER_IMAGE: Asset = manganis::asset!("/assets/images/join/azbel.png");
    const INDUSTRIALIST_IMAGE: Asset = manganis::asset!("/assets/images/join/rorqual.png");
    const SUPERCAPITAL_PILOT_IMAGE: Asset = manganis::asset!("/assets/images/join/avatar.png");

    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "flex flex-col gap-2 py-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl pb-4",
                        "What Does Your End Game Look Like?"
                    }
                    ul { class: "flex justify-evenly flex-wrap w-full",
                        li { class: "w-full sm:w-[500px] md:w-1/2 2xl:w-1/4 px-2 pb-4",
                            div { class: "card card-compact bg-base-100 shadow",
                                figure {
                                    img {
                                        alt: "Monitor",
                                        src: FLEET_COMMANDER_IMAGE
                                    }
                                }
                                div { class: "card-body",
                                    h2 { class: "card-title", "Fleet Commander" }
                                    p {
                                        "Leader of large scale 100+ player fleets fighting over strategic objectives?"
                                    }
                                }
                            }
                        }
                        li { class: "w-full sm:w-[500px] md:w-1/2 2xl:w-1/4 px-2 pb-4",
                            div { class: "card card-compact bg-base-100 shadow",
                                figure {
                                    img {
                                        alt: "Azbel",
                                        src: CORPORATION_LEADER_IMAGE
                                    }
                                }
                                div { class: "card-body",
                                    h2 { class: "card-title", "Corporation Leader" }
                                    p {
                                        "CEO of your own large scale organization or small tight-knit group?"
                                    }
                                }
                            }
                        }
                        li { class: "w-full sm:w-[500px] md:w-1/2 2xl:w-1/4 px-2 pb-4",
                            div { class: "card card-compact bg-base-100 shadow",
                                figure {
                                    img {
                                        alt: "Rorqual",
                                        src: INDUSTRIALIST_IMAGE
                                    }
                                }
                                div { class: "card-body",
                                    h2 { class: "card-title", "Industrialist" }
                                    p {
                                        "An industrialist critical to the economy of entire coalitions?"
                                    }
                                }
                            }
                        }
                        li { class: "w-full sm:w-[500px] md:w-1/2 2xl:w-1/4 px-2 pb-4",
                            div { class: "card card-compact bg-base-100 shadow",
                                figure {
                                    img {
                                        alt: "Avatar",
                                        src: SUPERCAPITAL_PILOT_IMAGE
                                    }
                                }
                                div { class: "card-body",
                                    h2 { class: "card-title", "Supercapital Pilot" }
                                    p {
                                        "Pilot of a legendary titan class super capital wielding a doomsday weapon?"
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "w-full sm:w-3/4 py-6",
                    div { class: "relative w-full pt-[56.25%]",
                        iframe {
                            class: "absolute w-full h-full top-0 left-0",
                            title: "This is EVE Video",
                            src: FEATURED_VIDEO,
                            allowfullscreen: true
                        }
                    }
                }
                div { class: "py-6",
                    h3 { class: "text-xl text-center",
                        "No matter what your end game is, Autumn's goal is to get you there."
                    }
                }
            }
        }
    }
}
