use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaDiscord, FaGithub};
use dioxus_free_icons::Icon;

use crate::web::constant::{DISCORD_URL, EVE_LEGAL_STATEMENT, GITHUB_URL};

#[component]
pub fn Footer() -> Element {
    const AUTUMN_LOGO: Asset = asset!("/assets/autumn-logo-dark.png");

    rsx! {
        footer { class: "footer footer-center bg-base-200 text-base-content p-6 md:p-10 justify-center",
            div { class: "max-w-[1440px] w-full flex justify-between flex-wrap",
                aside {
                    a {
                        href: "/",
                        class: "btn btn-ghost flex items-center gap-2",
                        img {
                            class: "w-12 h-12",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        p { class: "text-2xl font-bold", "Autumn" }
                    }
                }
                nav { class: "flex gap-2 w-full justify-between sm:w-fit sm:justify-end",
                    ul { class: "flex list-none",
                        li {
                            a {
                                class: "btn btn-square btn-ghost",
                                href: DISCORD_URL,
                                aria_label: "Discord",
                                Icon { width: 24, height: 24, icon: FaDiscord }
                            }
                        }
                        li {
                            a {
                                class: "btn btn-square btn-ghost",
                                href: GITHUB_URL,
                                aria_label: "GitHub",
                                Icon { width: 24, height: 24, icon: FaGithub }
                            }
                        }
                    }
                    a { href: "/join-autumn", class: "btn px-2 md:px-4 btn-primary", "Join Autumn" }
                }
            }
            p { class: "text-xs", {EVE_LEGAL_STATEMENT}}
        }
    }
}
