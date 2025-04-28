pub(crate) mod home;

use dioxus::prelude::*;

use home::*;

#[derive(Routable, Clone)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
}