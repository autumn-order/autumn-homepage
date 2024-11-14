use dioxus::prelude::*;
use dioxus_document::Link;

use crate::web::Route;

// Dioxus CLI has trouble serving favicon.ico from assets currently, this may just be a bug in Dioxus 0.6.0-alpha.4
// For now a RootLayout is used to direct the browser to the favicon
#[component]
pub fn RootLayout() -> Element {
    const FAVICON: Asset = manganis::asset!("./assets/favicon.ico");

    rsx! {
        Link {
            rel: "icon",
            href: FAVICON
        }
        Outlet::<Route> {}
    }
}
