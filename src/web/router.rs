use dioxus::prelude::*;

use crate::web::routes::join::JoinAutumn;
use crate::web::routes::{Home, NotFound, RootLayout};

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(RootLayout)]
        #[route("/")]
        Home {},
        #[route("/join-autumn")]
        JoinAutumn {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
}
