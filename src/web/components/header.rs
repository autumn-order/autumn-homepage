use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::{fa_brands_icons::FaDiscord, fa_solid_icons::FaBars};
use dioxus_free_icons::Icon;

use crate::web::constant::{DISCORD_URL, GITHUB_URL};

#[derive(PartialEq, Clone, Props)]
pub struct HeaderLink {
    pub text: &'static str,
    pub href: &'static str,
}

#[component]
pub fn Header() -> Element {
    const AUTUMN_LOGO: Asset = asset!("/assets/autumn-logo-dark.png");

    let links: Vec<HeaderLink> = vec![];

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100 z-20",
            div { class: "max-w-[1440px] w-full flex items-center justify-between px-6 py-3",
                ul { class: "flex gap-2 items-center",
                    li {
                        a {
                            class: "btn btn-ghost flex gap-2 items-center font-bold text-2xl",
                            href: "/",
                            img {
                                class: "w-12 h-12",
                                alt: "Autumn Logo",
                                src: AUTUMN_LOGO
                            }
                            "Autumn"
                        }
                    }
                    for (key , value) in links.iter().enumerate() {
                        li { key: "{key}",
                            a { class: "btn btn-ghost", href: value.href, "{value.text}" }
                        }
                    }
                }
                div {
                    ul { class: "hidden md:flex gap-2",
                        ul { class: "flex",
                            li {
                                a {
                                    class: "btn btn-square btn-ghost",
                                    href: DISCORD_URL,
                                    Icon { width: 24, height: 24, icon: FaDiscord }
                                }
                            }
                            li {
                                a {
                                    class: "btn btn-square btn-ghost",
                                    href: GITHUB_URL,
                                    Icon { width: 24, height: 24, icon: FaGithub }
                                }
                            }
                        }
                        li {
                            a {
                                href: "/join",
                                class: "btn px-2 md:px-4 btn-primary",
                                "Learn More"
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
                            for (key , value) in links.iter().enumerate() {
                                li { key: "{key}",
                                    a { href: value.href, "{value.text}" }
                                }
                            }
                            li {
                                a { href: "/join", "Learn More" }
                            }
                        }
                    }
                }
            }
        }
    }
}
