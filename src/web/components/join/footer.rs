use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::Icon;
use manganis::Asset;

use crate::web::constant::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn JoinFooter() -> Element {
    const AUTUMN_LOGO: Asset = manganis::asset!("assets/autumn-logo-dark.png");

    rsx! {
        footer { class: "footer footer-center bg-base-200 text-base-content p-6 md:p-10 justify-center",
            div { class: "max-w-[1440px] w-full",
                aside { class: "flex flex-col items-center",
                    a { href: "/join", class: "flex flex-col items-center",
                        img {
                            class: "w-32 h-32",
                            alt: "Autumn Logo",
                            src: AUTUMN_LOGO
                        }
                        p { class: "text-3xl font-bold", "Autumn" }
                    }
                }
                nav { class: "w-full",
                    ul { class: "flex w-full",
                        li { class: "w-1/2 pr-1 flex justify-end",
                            a {
                                href: DISCORD_URL,
                                class: "btn btn-outline px-2 md:px-4",
                                Icon { width: 20, height: 20, icon: FaDiscord }
                                "Autumn Discord"
                            }
                        }
                        li { class: "w-1/2 pl-1 flex justify-start",
                            a {
                                href: APPLICATIONS_URL,
                                class: "btn btn-primary px-2 md:px-4",
                                "Begin Your Journey"
                            }
                        }
                    }
                }
            }
            p { class: "text-xs",
                "EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights are reserved worldwide. All other trademarks are the property of their respective owners. EVE Online, the EVE logo, EVE and all associated logos and designs are the intellectual property of CCP hf. All artwork, screenshots, characters, vehicles, storylines, world facts or other recognizable features of the intellectual property relating to these trademarks are likewise the intellectual property of CCP hf. CCP hf. has granted permission to Autumn to use EVE Online and all associated logos and designs for promotional and information purposes on its website but does not endorse, and is not in any way affiliated with, Autumn. CCP is in no way responsible for the content on or functioning of this website, nor can it be liable for any damage arising from the use of this website."
            }
        }
    }
}
