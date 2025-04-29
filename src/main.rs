mod routes;

use dioxus::prelude::*;

const ICON: Asset = asset!("/assets/vauxl_icon.png");

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
            class: "bg-blue-600 text-white p-4 shadow-md font-main",
            div {
                class: "container mx-auto flex justify-between items-center",
                div{
                    class: "container mx-auto flex justify-start",
                    a {
                        class: "text-xl font-bold font-[sofia-pro-soft]",
                        href: "/",
                        img {
                        class: "mask-origin-content mask-x-from-70% mask-y-from-70%",
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
                    // Use 'class' for Tailwind classes in rsx!
                    class: "flex space-x-4", // Simplified class for demonstration
                    // Embed the iterator directly and use .ok() to convert Result<VNode, RenderError> to Option<VNode> (Element)
                    {
                        links.iter().map(|(title, url)| rsx! {
                            a {
                                // Add a 'key' prop for efficient list rendering (important!).
                                key: "{url}",
                                href: "{url}",
                                // Use 'class' again for link styles.
                                class: "rounded-lg px-3 py-2 text-white font-medium hover:bg-blue-700 hover:text-gray-100 transition duration-300", // Adjusted colors for header
                                // Embed the title text using curly braces.
                                "{title}"
                            }
                        }.ok()) // <--- Add .ok() here to handle the Result
                    }
                },
            },
        },
    }
}

#[component]
fn vauxl_footer() -> Element {
    // Return the rsx! block directly instead of using cx.render!
    rsx! {
        footer {
            class: "bg-gray-800 text-gray-300 p-6 text-center shadow-inner",
            div {
                class: "container mx-auto",
                p {
                    class: "text-sm",
                    "© 2025 VauxlNet. All rights reserved."
                }
                // You can add more info or links here
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
                }
            }
        }
    }
}
