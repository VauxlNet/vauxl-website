pub(crate) mod home;
pub(crate) mod error_404;

use dioxus::prelude::*;

use home::*;
use error_404::*;

#[derive(Routable, Clone)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    
    #[route("/:..route")]
    Error404 { route: Vec<String> },
}