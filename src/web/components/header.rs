use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::Icon;
use manganis::ImageAsset;

use crate::constants::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn Header() -> Element {
    const AUTUMN_LOGO: ImageAsset = manganis::mg!(image("assets/autumn-logo.png")
        .format(ImageType::Avif)
        .size(64, 64));

    rsx! {
        header { class: "flex justify-center bg-base-100",
            div { class: "navbar max-w-[1440px] w-full flex justify-between",
                ul {
                    li {
                        a { class: "btn btn-lg btn-ghost text-3xl", href: "/",
                            img { class: "w-16 h-16", src: AUTUMN_LOGO }
                            "Autumn"
                        }
                    }
                }
                ul { class: "flex gap-2",
                    li {
                        a { class: "btn btn-square", href: DISCORD_URL,
                            Icon { width: 32, height: 32, fill: "black", icon: FaDiscord }
                        }
                    }
                    li {
                        a { class: "btn btn-primary", href: APPLICATIONS_URL, "Begin Your Journey" }
                    }
                }
            }
        }
    }
}
