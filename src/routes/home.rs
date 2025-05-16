use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            class: "flex flex-col gap-2",
            div {
                class: "container mx-auto p-6 bg-main-200 rounded-lg shadow-xl mt-10 mb-10 grid grid-cols-3 grid-rows-6",
                h1 {
                    class: "text-5xl font-bold text-main-300 text-left row-1",
                    "Your Server."
                }
                h1 {
                    class: "text-5xl font-bold text-main-300 text-left row-2",
                    "Your Rules."
                }
                h1 {
                    class: "text-5xl font-bold text-main-300 text-left row-3",
                    "Your Data."
                }
                h1 {
                    class: "text-main-400 text-lg text-left font-light row-4",
                    "with Vauxl."
                }
            },
            div {
                class: "container mx-auto grid grid-cols-6 text-main-500",
                a {
                    class: "m-4 px-4 py-4 bg-main-400 font-semibold rounded-md shadow-md hover:bg-main-500 hover:text-main-300 transition duration-300 block grid col-start-2 col-end-4 justify-self-end",
                    href: "https://app.vauxl.net",
                    "Open Vauxl in your Browser"
                }
                button {
                    class: "m-4 px-4 py-4 bg-main-400 font-semibold rounded-md shadow-md hover:bg-main-500 hover:text-main-300 transition duration-300 block grid col-start-4 col-end-6 justify-self-start",
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