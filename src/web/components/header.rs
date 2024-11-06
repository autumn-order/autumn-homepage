use dioxus::prelude::*;
use dioxus_free_icons::icons::{fa_brands_icons::FaDiscord, fa_solid_icons::FaBars};
use dioxus_free_icons::Icon;
use manganis::ImageAsset;

use crate::web::constants::{APPLICATIONS_URL, DISCORD_URL};

#[component]
pub fn Header() -> Element {
    const AUTUMN_LOGO: ImageAsset = manganis::mg!(image("assets/autumn-logo-dark.png")
        .format(ImageType::Avif)
        .size(64, 64));

    rsx! {
        header { class: "fixed w-full flex justify-center bg-base-100 z-20",
            div { class: "max-w-[1440px] w-full flex items-center justify-between px-6 py-3",
                ul {
                    li {
                        a {
                            class: "flex gap-2 items-center font-bold text-3xl",
                            href: "/",
                            img {
                                class: "w-16 h-16",
                                alt: "Autumn Logo",
                                src: AUTUMN_LOGO
                            }
                            "Autumn"
                        }
                    }
                }
                ul { class: "hidden md:flex gap-2",
                    li {
                        a { class: "btn btn-outline", href: DISCORD_URL,
                            Icon { width: 24, height: 24, icon: FaDiscord }
                            "Autumn Discord"
                        }
                    }
                    li {
                        a { class: "btn btn-primary", href: APPLICATIONS_URL, "Begin Your Journey" }
                    }
                }
                details { class: "dropdown dropdown-end flex md:hidden",
                    summary { class: "btn btn-square btn-ghost",
                        Icon { width: 24, height: 24, icon: FaBars }
                    }
                    ul { class: "menu dropdown-content bg-base-100 w-52 rounded-b",
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
