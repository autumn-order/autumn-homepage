use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::Icon;
use manganis::ImageAsset;

use crate::constants::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn Header() -> Element {
    const AUTUMN_LOGO: ImageAsset = manganis::mg!(image("assets/autumn-logo-dark.png")
        .format(ImageType::Avif)
        .size(64, 64));

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100",
            div { class: "max-w-[1440px] w-full flex items-center justify-between px-6 py-3",
                ul {
                    li {
                        a {
                            class: "flex gap-2 items-center font-bold text-3xl",
                            href: "/",
                            img { class: "w-16 h-16", src: AUTUMN_LOGO }
                            "Autumn"
                        }
                    }
                }
                ul { class: "hidden md:flex gap-2",
                    li {
                        a { class: "btn", href: DISCORD_URL,
                            Icon { width: 24, height: 24, fill: "black", icon: FaDiscord }
                            "Autumn Discord"
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
