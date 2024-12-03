use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::icons::fa_solid_icons::FaBars;
use dioxus_free_icons::Icon;
use manganis::Asset;

use crate::web::constant::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn JoinHeader() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!("/assets/autumn-logo-dark.avif");

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100 z-20 border-b border-base-200",
            div { class: "max-w-[1440px] w-full flex items-center justify-between px-6 py-3",
                ul { class: "flex gap-2 items-center",
                    li {
                        a {
                            class: "btn btn-ghost flex gap-2 items-center font-bold text-2xl",
                            href: "/join-autumn",
                            img {
                                class: "w-12 h-12",
                                alt: "Autumn Logo",
                                src: AUTUMN_LOGO
                            }
                            "Autumn"
                        }
                    }
                }
                div {
                    ul { class: "hidden md:flex gap-2",
                        li {
                            a { class: "btn btn-outline", href: DISCORD_URL,
                                Icon { width: 24, height: 24, icon: FaDiscord }
                                "Autumn Discord"
                            }
                        }
                        li {
                            a {
                                class: "btn btn-primary",
                                href: APPLICATIONS_URL,
                                "Begin Your Journey"
                            }
                        }
                    }
                    div { class: "dropdown dropdown-end md:hidden",
                        div {
                            tabindex: 0,
                            role: "button",
                            class: "btn btn-square btn-ghost",
                            Icon { width: 24, height: 24, icon: FaBars }
                        }
                        ul {
                            tabindex: 0,
                            class: "menu dropdown-content bg-base-100 w-52 rounded-b",
                            li {
                                a { href: DISCORD_URL,
                                    Icon { width: 24, height: 24, icon: FaDiscord }
                                    "Autumn Discord"
                                }
                            }
                            li {
                                a { href: APPLICATIONS_URL, "Begin Your Journey" }
                            }
                        }
                    }
                }
            }
        }
    }
}
