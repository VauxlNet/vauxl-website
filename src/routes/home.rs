use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            class: "flex flex-col gap-2",
            div {
                class: "container mx-auto p-6 bg-gray-100 rounded-lg shadow-xl mt-10 mb-10 grid grid-cols-3 grid-rows-5",
                h1 {
                    class: "text-5xl font-bold text-blue-020 mb-4 text-left row-span-2",
                    "Privatize your Conversation"
                }
                p {
                    class: "text-blue-010 text-lg text-left font-light row-start-3 row-span-3",
                    "with Vauxl"
                }
                br {}
            },
            div {
                class: "container mx-auto grid grid-cols-6",
                a {
                    class: "m-4 px-4 py-4 bg-blue-200 font-semibold rounded-md shadow-md hover:bg-blue-400 transition duration-300 block grid col-start-2 col-end-4 justify-self-end",
                    href: "https://app.vauxl.net",
                    "Open Vauxl in your Browser"
                }
                button {
                    class: "m-4 px-4 py-4 bg-blue-200 font-semibold rounded-md shadow-md hover:bg-blue-400 transition duration-300 block grid col-start-4 col-end-6 justify-self-start",
                    onclick: move |_| {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(&"AppOpenInBrowser".into());

                        #[cfg(not(target_arch = "wasm32"))]
                        println!("Browser");
                    },
                    "Download for OS"
                }
            }
        },
    }
}