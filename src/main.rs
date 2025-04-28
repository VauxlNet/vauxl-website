use dioxus::prelude::*;

fn main() {
    LaunchBuilder::new()
        .with_cfg(dioxus_web::Config::new())
        .launch(app);
}

fn app() -> Element {
    rsx! {
        vauxl_header {},

        div {
            class: "container mx-auto p-6 bg-gray-100 rounded-lg shadow-xl mt-10 mb-10",
            h1 {
                class: "text-3xl font-bold text-blue-700 mb-4 text-center",
                "Hello, Dioxus with Tailwind!"
            }
            p {
                class: "text-gray-800 text-lg text-center",
                "This is a basic example running with dioxus-cli and Tailwind CSS."
            }
            button {
                class: "mt-6 px-6 py-3 bg-green-500 text-white font-semibold rounded-md shadow-md hover:bg-green-600 transition duration-300 block mx-auto",
                onclick: move |_| {
                    // Use web_sys::console::log_1 for browser console output in web builds (Wasm)
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Button clicked!".into());
                    // Use println! for server-side or desktop builds
                    #[cfg(not(target_arch = "wasm32"))]
                    println!("Button clicked!");
                },
                "Click Me!"
            }
        },

        vauxl_footer {},
    }
}

#[component]
fn vauxl_header() -> Element {
    let links = vec![
        ("Home", "/dashboard"),
        ("Team", "/team"),
        ("Projects", "/projects"),
        ("Reports", "/reports"),
    ];

    rsx! {
        header {
            class: "bg-blue-600 text-white p-4 shadow-md",
            div {
                class: "container mx-auto flex justify-between items-center",
                div {
                    class: "text-xl font-bold",
                    "VauxlNet"
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
                }
            }
        }
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
                    "© 2023 VauxlNet. All rights reserved."
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
