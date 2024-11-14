use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{FaDiscord, FaGithub};
use dioxus_free_icons::Icon;
use manganis::Asset;

use crate::web::constant::{DISCORD_URL, GITHUB_URL};

#[component]
pub fn Footer() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!("assets/autumn-logo-dark.png");

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
                                href: DISCORD_URL,
                                class: "btn btn-ghost btn-square",
                                Icon { width: 24, height: 24, icon: FaDiscord }
                            }
                        }
                        li {
                            a {
                                href: GITHUB_URL,
                                class: "btn btn-ghost btn-square",
                                Icon { width: 24, height: 24, icon: FaGithub }
                            }
                        }
                    }
                    a { href: "/join", class: "btn px-2 md:px-4 btn-primary", "Join Autumn" }
                }
            }
            p { class: "text-xs",
                "EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights are reserved worldwide. All other trademarks are the property of their respective owners. EVE Online, the EVE logo, EVE and all associated logos and designs are the intellectual property of CCP hf. All artwork, screenshots, characters, vehicles, storylines, world facts or other recognizable features of the intellectual property relating to these trademarks are likewise the intellectual property of CCP hf. CCP hf. has granted permission to Autumn to use EVE Online and all associated logos and designs for promotional and information purposes on its website but does not endorse, and is not in any way affiliated with, Autumn. CCP is in no way responsible for the content on or functioning of this website, nor can it be liable for any damage arising from the use of this website."
            }
        }
    }
}
