use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::FaDiscord;
use dioxus_free_icons::Icon;
use manganis::ImageAsset;

use crate::web::constants::{BLACK_ROSE_WEBSITE_URL, DISCORD_URL};

#[component]
pub fn Hero() -> Element {
    const AUTUMN_LOGO: ImageAsset = manganis::mg!(image("./assets/autumn-logo.png")
        .format(ImageType::Avif)
        .size(512, 512));

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
                            "The Order of Autumn"
                        }
                        h2 { class: "text-white sm:text-lg lg:text-xl xl:text-2xl",
                            "An EVE Online nullsec corporation part of "
                            a {
                                class: "link",
                                href: BLACK_ROSE_WEBSITE_URL,
                                target: "_blank",
                                "Black Rose"
                            }
                            " alliance & Phoenix Coalition"
                        }
                        p { class: "text-white text-xs md:text-base",
                            "Autumn is a real life first, new player focused organization with a heavy emphasis on organization, community, and high quality IT infrastructure."
                        }
                    }
                    ul { class: "flex flex-wrap gap-2 justify-center",
                        li {
                            a { href: DISCORD_URL, class: "btn px-2 md:px-4",
                                Icon { width: 24, height: 24, icon: FaDiscord }
                                "Autumn Discord"
                            }
                        }
                        li {
                            a {
                                href: "/join",
                                class: "btn px-2 md:px-4 btn-primary",
                                "Join Autumn"
                            }
                        }
                    }
                }
            }
        }
    }
}
