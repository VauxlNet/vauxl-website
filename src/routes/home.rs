use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            class: "flex flex-col gap-2",
            div {
                class: "container mx-auto p-6 bg-main-100 rounded-lg shadow-xl mt-10 mb-2 grid gap-2 grid-cols-8 grid-rows-5",
                h1 {
                    class: "text-5xl mb-2 font-bold text-main-300 text-left row-2 col-span-2",
                    "Vauxl:"
                }
                h1 {
                    class: "text-4xl font-bold text-main-300 text-left row-3 col-span-3",
                    "Your Communication, Reclaimed!"
                }
                h1 {
                    class: "text-4xl font-bold text-main-300 text-left row-4 col-span-4",
                    "Private, Secure, Customizable."
                }
                
            },
            div {
                class: "container mx-auto grid grid-cols-6 text-main-300 -mt-2",
                a {
                    class: "m-4 px-4 py-4 bg-main-500 font-semibold rounded-md shadow-md hover:bg-main-400 transition duration-300 block grid col-start-2 col-end-4 justify-self-end",
                    href: "https://app.vauxl.net",
                    "Open Vauxl in your Browser"
                }
                button {
                    class: "m-4 px-4 py-4 bg-main-500 font-semibold rounded-md shadow-md hover:bg-main-400 transition duration-300 block grid col-start-4 col-end-6 justify-self-start",
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