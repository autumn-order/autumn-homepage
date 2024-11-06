use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::{
        fa_brands_icons::FaDiscord,
        fa_solid_icons::{FaSkullCrossbones, FaUsers},
    },
    Icon,
};

use crate::web::{
    constants::{APPLICATIONS_URL, CORPORATIONS, DISCORD_URL},
    model::corporation::CorpCardData,
};

#[component]
pub fn CallToAction() -> Element {
    #[derive(PartialEq, Clone, Props)]
    struct CorporationCardProps {
        corporation: &'static CorpCardData,
    }

    fn CorporationCard(props: CorporationCardProps) -> Element {
        rsx!(
            div { class: "card card-compact shadow h-96 min-w-64 max-w-72",
                div { class: "card-body flex flex-col justify-between items-center text-center",
                    img {
                        class: "avatar w-24 h-24",
                        src: format!(
                            "https://images.evetech.net/corporations/{}/logo?size=128",
                            props.corporation.corporation_id,
                        ),
                        alt: format!("{} Logo", props.corporation.name)
                    }
                    div { class: "flex flex-col gap-2",
                        p { class: "font-bold", "{props.corporation.name}" }
                        p { "{props.corporation.location}" }
                    }
                    ul { class: "flex justify-evenly w-full",
                        li { class: "flex flex-col items-center gap-2 w-1/2",
                            Icon { width: 24, height: 24, fill: "black", icon: FaUsers }
                            p { "Members" }
                            p { "{props.corporation.members}" }
                        }
                        li { class: "flex flex-col items-center gap-2 w-1/2",
                            Icon { width: 24, height: 24, fill: "black", icon: FaSkullCrossbones }
                            p { "Ships Destroyed" }
                            p { "{props.corporation.ships_destroyed}" }
                        }
                    }
                    a { href: APPLICATIONS_URL, class: "btn", { props.corporation.cta_text } }
                }
            }
        )
    }

    rsx! {
        section { class: "flex items-center justify-center",
            div { class: "max-w-[1440px] p-6 w-full h-full flex flex-col items-center",
                div { class: "w-full pt-6",
                    h1 { class: "text-center font-bold text-2xl xl:text-4xl",
                        "Begin Your Journey as Early as Right Now"
                    }
                }
                div { class: "w-full py-6 flex flex-wrap gap-4 md:gap-0",
                    div { class: "w-full flex flex-col items-center xl:w-1/2",
                        h2 { class: "font-bold text-center text-xl md:text-2xl py-6",
                            "Join Many Others in Taking that First Step"
                        }
                        div { class: "card card-compact h-64 sm:h-96 w-full max-w-[696px] shadow" }
                    }
                    div { class: "w-full xl:w-1/2",
                        h2 { class: "font-bold text-center text-xl md:text-2xl py-6",
                            "Join Autumn in Nullsec or Highsec"
                        }
                        ul { class: "flex flex-wrap justify-center",
                            li { class: "py-2 px-8 md:pr-2 md:py-0",
                                CorporationCard { corporation: &CORPORATIONS[0] }
                            }
                            li { class: "py-2 px-8 md:pl-2 md:py-0",
                                CorporationCard { corporation: &CORPORATIONS[1] }
                            }
                        }
                    }
                }
                div { class: "w-full py-6 flex-wrap md:flex-nowrap flex gap-4 md:gap-0",
                    div { class: "w-full flex flex-col items-center",
                        h2 { class: "font-bold text-center text-xl md:text-2xl pb-6",
                            "Chat with a Recruiter"
                        }
                        div { class: "flex flex-col gap-2",
                            h3 { class: "font-bold text-center text-lg", "Reach us Through Either" }
                            div { class: "flex justify-center",
                                a { href: DISCORD_URL, class: "btn",
                                    Icon { width: 24, height: 24, fill: "black", icon: FaDiscord }
                                    "The Autumn Discord"
                                }
                            }
                            p {
                                "The "
                                b { "Autumn Public" }
                                " in-game chat channel"
                            }
                            p { "EVE Mail or Discord message any of our leadership below" }
                        }
                    }
                    div {
                        span { class: "font-bold text-center text-2xl hidden md:block px-4",
                            "OR"
                        }
                    }
                    div { class: "w-full",
                        h2 { class: "font-bold text-center text-xl md:text-2xl pb-6",
                            "Begin the Application Process"
                        }
                        ul { class: "flex flex-col gap-2 px-10",
                            li {
                                "1. Submit your application at "
                                a {
                                    class: "link-primary",
                                    href: APPLICATIONS_URL,
                                    { APPLICATIONS_URL }
                                }
                            }
                            li {
                                "2. Wait for a recruiter to review your application, come chat with us in Discord or Autumn Public while you wait!"
                            }
                            li {
                                "3. Accept your invitation, most applications are reviewed in less than 24 hours."
                            }
                            li {
                                "4. Follow our Getting Started Guide to get setup and come take part in our community!"
                            }
                        }
                    }
                }
                ul { class: "w-full py-6 flex justify-center",
                    li {
                        div { class: "card card-compact shadow w-72 md:w-96 md:h-48 lg:h-32",
                            div { class: "card-body flex-row gap-4",
                                img {
                                    class: "avatar w-24 h-24 rounded-full",
                                    src: "https://images.evetech.net/characters/2114794365/portrait?size=128"
                                }
                                div { class: "flex flex-col justify-between",
                                    p { class: "font-bold flex items-center", "CEO of Autumn" }
                                    p { class: "font-bold flex items-center", "Hyziri" }
                                    div { class: "flex items-center gap-1",
                                        Icon { width: 20, height: 20, fill: "black", icon: FaDiscord }
                                        p { "hyziri" }
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
