use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
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
    }
}