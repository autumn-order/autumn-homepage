use dioxus::prelude::*;

use crate::web::routes::join::JoinAutumn;
use crate::web::routes::Layout;
use crate::web::routes::{Home, NotFound};

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Layout)]
        #[route("/")]
        Home {},
        #[route("/:..segments")]
        NotFound { segments: Vec<String> },
    #[end_layout]
    #[route("/join-autumn")]
    JoinAutumn {},
}
