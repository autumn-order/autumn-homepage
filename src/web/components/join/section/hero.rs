use dioxus::prelude::*;
use dioxus_free_icons::icons::{fa_brands_icons::FaDiscord, fa_solid_icons::FaChevronDown};
use dioxus_free_icons::Icon;
use manganis::Asset;

use crate::web::constant::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn Hero() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!("./assets/autumn-logo.png");

    rsx! {
        section { class: "flex items-center justify-center bg-gradient-to-br from-orange-950 to-amber-800 h-screen pt-[88px] pb-6",
            div { class: "max-w-[1440px] px-6 w-full h-full flex flex-col items-center",
                div { class: "md:w-3/4 flex flex-col items-center md:items-start md:self-start gap-4 my-auto",
                    div { class: "flex flex-col text-center md:text-left items-center md:items-start gap-2",
                        img {
                            class: "w-48 h-48 md:w-64 md:h-64",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        h1 { class: "text-white font-bold sm:text-2xl lg:text-3xl xl:text-4xl",
                            "EVE is complicated, Autumn makes it straightforward."
                        }
                        h2 { class: "text-white sm:text-lg lg:text-xl xl:text-2xl",
                            "There are many twists and turns in the beginning of an EVE journey, why waste time learning the hard way when you can learn the right way?"
                        }
                        p { class: "text-white text-xs md:text-base",
                            "Begin your journey in nullsec with The Order of Autumn, a corporation part of Black Rose alliance & Phoenix Coalition, or in highsec with Autumn Highsec Division."
                        }
                    }
                    ul { class: "flex gap-2",
                        li {
                            a { href: DISCORD_URL, class: "btn px-2 md:px-4",
                                Icon { width: 24, height: 24, icon: FaDiscord }
                                "Autumn Discord"
                            }
                        }
                        li {
                            a {
                                href: APPLICATIONS_URL,
                                class: "btn px-2 md:px-4 btn-primary",
                                "Begin Your Journey"
                            }
                        }
                    }
                }
                div { class: "flex flex-col justify-self-end items-center text-white",
                    span { class: "font-bold text-xl md:text-2xl", "Learn More" }
                    Icon { width: 24, height: 24, icon: FaChevronDown }
                }
            }
        }
    }
}
