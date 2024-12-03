use super::Route;
use dioxus::prelude::*;
use dioxus_document::{Link, Meta};

#[component]
pub fn App() -> Element {
    const FAVICON: Asset = manganis::asset!("/assets/favicon.ico");
    const AUTUMN_LOGO: Asset = manganis::asset!(
        "/assets/autumn-logo.avif",
        ImageAssetOptions::new()
            .with_avif()
            .with_size(ImageSize::Manual {
                width: 256,
                height: 256
            })
    );

    rsx! {
        Link {
            rel: "icon",
            href: FAVICON
        }
        Meta {
            name: "og:image",
            content: AUTUMN_LOGO
        }
        Meta {
            name: "twitter:image",
            content: AUTUMN_LOGO
        }
        Router::<Route> {}
    }
}
