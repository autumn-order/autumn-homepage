use dioxus::prelude::*;

use crate::web::routes::join::JoinAutumn;
use crate::web::routes::Home;

use crate::web::routes::layout::RootLayout;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Home {},
        #[route("/join")]
        JoinAutumn {},
}
