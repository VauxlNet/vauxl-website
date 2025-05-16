use dioxus::prelude::*;

#[component]
pub fn Error404(route: Vec<String>) -> Element {
    rsx! {
        div{
                class:"flex-grow",
        }
        div{
            class:"flex-none items-center mx-auto bg-main-400 rounded-lg p-4 text-main-100",
            h1 { "Page not found" }
            p { "We are terribly sorry, but the page you requested doesn't exist." }
            pre { 
                color: "red", "log:\nattemped to navigate to: {route:?}" 
            }
        }
    }
}