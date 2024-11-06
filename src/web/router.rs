use dioxus::prelude::*;

use crate::web::routes::join::JoinAutumn;
use crate::web::routes::Home;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/join")]
    JoinAutumn {},
}
