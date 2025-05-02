mod routes;

use dioxus::prelude::*;

const ICON: Asset = asset!("/assets/vauxl_icon03.png");

fn main() {
    LaunchBuilder::new()
        .with_cfg(dioxus_web::Config::new())
        .launch(app);
}

fn app() -> Element {
    rsx! {
        document::Stylesheet{
            href: asset!("./public/output.css")
        }

        div{
            class: "flex flex-col h-dvh font-main",

            vauxl_header {},

            Router::<routes::Route> {},

            div{
                class:"flex-grow",
            }

            vauxl_footer {},
        },
    }
}

#[component]
fn vauxl_header() -> Element {
    let links = vec![
        ("Team", "/team"),
        ("Projects", "/projects"),
        ("Reports", "/contact/reports"),
        ("Dashboard", "/dashboard"),
    ];

    rsx! {
        header {
            class: "bg-blue-040 text-white p-4 shadow-md font-main",
            div {
                class: "container mx-auto flex justify-between items-center",
                div{
                    class: "container mx-auto flex justify-start",
                    a {
                        class: "mr-2",
                        href: "/",
                        img {
                        class: "mask-origin-content",
                        src: ICON,
                        height: "30px",
                        width: "30px",
                        },
                    },
                    a {
                        class: "text-xl font-bold mr-8",
                        href: "/",
                        "VauxlNet",
                    },
                }
                nav {
                    class: "flex space-x-4",
                    {
                        links.iter().map(|(title, url)| rsx! {
                            a {
                                key: "{url}",
                                href: "{url}",
                                class: "rounded-lg px-3 py-2 text-white font-medium hover:bg-blue-030 hover:text-gray-100 transition duration-300",
                                "{title}"
                            }
                        }.ok())
                    }
                },
            },
        },
    }
}

#[component]
fn vauxl_footer() -> Element {
    rsx! {
        footer {
            class: "bg-blue-040 text-gray-300 p-6 text-center shadow-inner",
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
