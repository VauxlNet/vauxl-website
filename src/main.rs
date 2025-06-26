mod routes;
mod header;

use dioxus::prelude::*;

//maybe for use in later things i dont know
//const ICON: Asset = asset!("/assets/vauxl_icon03.png");

fn main() {
    launch(app)
}

fn app() -> Element {
    rsx! {
        document::Stylesheet{
            href: asset!("./public/output.css")
        }

        div{
            class: "flex flex-col h-dvh font-main bg-gray-600 tracking-wide",

            header::Header {},

            Router::<routes::Route> {},

            div{
                class:"flex-grow",
            }

            vauxl_footer {},
        },
    }
}

#[component]
fn vauxl_footer() -> Element {
    rsx! {
        footer {
            class: "bg-main-100 text-main-500 p-6 text-center shadow-inner",
            div {
                class: "container mx-auto",
                p {
                    class: "text-sm",
                    "© 2025 VauxlNet. All rights reserved."
                }
                div {
                    class: "mt-2 text-sm",
                    a {
                        href: "/privacy",
                        class: "hover:underline mx-2",
                        "Privacy Policy"
                    }
                    a {
                        href: "/terms",
                        class: "hover:underline mx-2",
                        "Terms of Service"
                    }
                    a {
                        href: "/contact",
                        class: "hover:underline mx-2",
                        "Contact"
                    }
                }
            }
        }
    }
}
