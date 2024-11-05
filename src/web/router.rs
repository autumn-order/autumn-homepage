use dioxus::prelude::*;

use crate::web::routes::Home;
use crate::web::routes::Layout;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
}
