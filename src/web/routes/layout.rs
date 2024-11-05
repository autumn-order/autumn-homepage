use dioxus::prelude::*;

use crate::web::components::{Footer, Header};
use crate::web::Route;

#[component]
pub fn Layout() -> Element {
    rsx! {
        Header {}
        Outlet::<Route> {}
        Footer {}
    }
}
