use dioxus::prelude::*;

#[component]
pub fn Error404(route: Vec<String>) -> Element {
    rsx! {
        div{
            class:"flex-grow items-center mx-auto",
            h1 { "Page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
        }
    }
}