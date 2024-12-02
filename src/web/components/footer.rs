use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_brands_icons::{
    FaBluesky, FaDiscord, FaGithub, FaReddit, FaSoundcloud, FaTwitch, FaYoutube,
};
use dioxus_free_icons::Icon;

use crate::web::constant::{
    BLUESKY_URL, DISCORD_URL, GITHUB_URL, REDDIT_URL, SOUNDCLOUD_URL, TWITCH_URL, YOUTUBE_URL,
};

pub struct FooterSection {
    pub title: &'static str,
    pub links: Vec<FooterLink>,
}

#[derive(PartialEq, Clone, Props)]
pub struct FooterLink {
    pub text: &'static str,
    pub href: &'static str,
}

#[component]
pub fn Footer() -> Element {
    const AUTUMN_LOGO: Asset = asset!("/assets/autumn-logo-dark.png");

    let footer_sections: Vec<FooterSection> = vec![
        FooterSection {
            title: "Services",
            links: vec![
                FooterLink {
                    text: "Apply",
                    href: "https://apply.autumn-order.com",
                },
                FooterLink {
                    text: "SeAT",
                    href: "https://seat.autumn-order.com",
                },
            ],
        },
        FooterSection {
            title: "Information",
            links: vec![
                FooterLink {
                    text: "Wiki",
                    href: "https://wiki.autumn-order.com",
                },
                FooterLink {
                    text: "Roadmap",
                    href: "https://trello.com/b/2kdvKXnu/autumn-roadmap",
                },
            ],
        },
    ];

    rsx! {
        div { class: "flex flex-col items-center w-full py-10 bg-base-200",
            footer { class: "footer text-base-content px-4 max-w-[1440px] w-full flex gap-12 flex-wrap",
                aside { class: "w-full sm:w-fit",
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
                for (key, value) in footer_sections.iter().enumerate() {
                    nav { key: "{key}",
                        h6 { class: "footer-title", "{value.title}" }
                        for (key, value) in value.links.iter().enumerate() {
                            a { key: "{key}", class: "link link-hover", href: value.href, "{value.text}" }
                        }
                    }
                }
            }
            footer { class: "footer text-base-content px-4 pt-12 max-w-[1440px] w-full",
                ul { class: "flex flex-wrap gap-4",
                    li {
                        a { href: DISCORD_URL, class: "footer-media-link hover:bg-[#5865F2]",
                            Icon { width: 24, height: 24, icon: FaDiscord }

                        }
                    }
                    li {
                        a { href: YOUTUBE_URL, class: "footer-media-link hover:bg-[#FF0000]",
                            Icon { width: 24, height: 24, icon: FaYoutube }

                        }
                    }
                    li {
                        a { href: TWITCH_URL, class: "footer-media-link hover:bg-[#9146FF]",
                            Icon { width: 24, height: 24, icon: FaTwitch }

                        }
                    }
                    li {
                        a { href: BLUESKY_URL, class: "footer-media-link hover:bg-[#1D9BF0]",
                            Icon { width: 24, height: 24, icon: FaBluesky }

                        }
                    }
                    li {
                        a { href: REDDIT_URL, class: "footer-media-link hover:bg-[#FF4500]",
                            Icon { width: 24, height: 24, icon: FaReddit }

                        }
                    }
                    li {
                        a { href: GITHUB_URL, class: "footer-media-link hover:bg-[#000000]",
                            Icon { width: 24, height: 24, icon: FaGithub }

                        }
                    }
                    li {
                        a { href: SOUNDCLOUD_URL, class: "footer-media-link hover:bg-[#F26F23]",
                            Icon { width: 24, height: 24, icon: FaSoundcloud }

                        }
                    }
                }
            }
        }
    }
}
