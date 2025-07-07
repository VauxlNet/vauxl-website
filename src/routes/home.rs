use dioxus::prelude::*;

const TEMP_IMAGE: Asset = asset!("/assets/image.webp");

#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            id: "hero",
            class: "flex flex-col gap-2",
            div {
                class: "container mx-auto p-6 bg-main-100 rounded-lg shadow-xl mt-10 mb-2 grid grid-cols-2 gap-4 h-[600px]",
                div {
                    class: "col-span-1 flex flex-col justify-center p-4",
                    h1{
                        class: "text-5xl mb-2 font-bold text-main-300 text-left",
                        "Vauxl:",
                    },
                    h1 {
                        class: "text-4xl font-bold text-main-300 text-left",
                        "Your Communication, Reclaimed!",
                    },
                    h1 {
                        class: "text-4xl font-bold text-main-300 text-left",
                        "Private, Secure, Customizable.",
                    },
                },
                div{
                    class: "col-span-1 relative flex items-center justify-center hidden md:flex",
                    img{
                        class: "absolute right-[50px] top-1/2 -translate-y-1/2 w-[800px] h-[450px] bg-black shadow-2xl rounded-lg overflow-hidden perspective-[75em] -rotate-y-20",
                        src: TEMP_IMAGE,
                    },
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