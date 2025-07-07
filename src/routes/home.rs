use dioxus::prelude::*;

const TEMP_IMAGE: Asset = asset!("/assets/image.webp");

#[component]
pub fn Home() -> Element {
    rsx! {
        div{
            class: "flex flex-col gap-2 bg-gray-600 font-main",
            div {
                id: "hero",
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
            div {
                id: "rollingSections",
                class: "flex flex-col gap-2 container mx-auto p-6 bg-main-100 rounded-lg shadow-xl mb-2 text-main-300",
                section {
                    h2 {
                        class: "text-3xl font-bold my-2",
                        "Why Vauxl?"
                    }
                    dl {
                        class: "divide-y divide-main-200",
                        div{
                            class: "px-4 py-6",
                            dt {
                                class: "font-bold text-xl",
                                "Privacy-by-Design"
                            }
                            dd {
                                details {
                                    class: "border border-transparent open:border-main-400 open:rounded-lg open:p-2 open:bg-main-500 mt-1 text-m font-semibold",
                                    summary {
                                        class: " leading-6 select-none",
                                        "Your data, your control. Compliant with European privacy policy laws and rules and prioritizing European legal standards first, then global best practices."
                                    }
                                    p {
                                        class: "mt-3 leading-6 text-main-100",
                                        "We believe your conversations are yours. Vauxl is engineered from the ground up to minimize data collection and empower you with control over your information, including end-to-end encrypted direct messages. Our commitment to privacy is deeply rooted in European legal standards, which we prioritize as our foundational benchmark for user data protection."
                                    }
                                }
                            }
                        }
                        div{
                            class: "px-4 py-6",
                            dt {
                                class: "font-bold",
                                "Unrivaled Security"
                            }
                            dd {
                                details {
                                    class: "border border-transparent open:border-main-400 open:rounded-lg open:p-2 open:bg-main-500 mt-1 text-m font-semibold",
                                    summary {
                                        class: " leading-6 select-none",
                                        "Built with Rust, TLS 1.3, E2EE for DMs. Adhering to strict European security standards."
                                    }
                                    p {
                                        class: "mt-3 leading-6 text-main-100",
                                        "Built in Rust, Vauxl leverages cutting-edge cryptographic standards like TLS 1.3 and Argon2 to ensure your communications are always protected. We adhere to the highest security protocols, starting with the robust legal requirements in Europe, and then extending to global best practices to ensure your data's integrity and confidentiality."
                                    }
                                }
                            }
                        }
                        div{
                            class: "px-4 py-6",
                            dt {
                                class: "font-bold",
                                "Deep Customization"
                            }
                            dd {
                                details {
                                    class: "border border-transparent open:border-main-400 open:rounded-lg open:p-2 open:bg-main-500 mt-1 text-m font-semibold",
                                    summary {
                                        class: " leading-6 select-none",
                                        "Themes & powerful Wasm Plugins"
                                    }
                                    p {
                                        class: "mt-3 leading-6 text-main-100",
                                        "Make Vauxl truly yours. With a flexible theming engine and a secure WebAssembly plugin architecture, you can tailor your experience to your exact needs."
                                    }
                                }
                                
                            }
                        }
                        div {
                            class: "px-4 py-6",
                            dt {
                                class: "font-bold",
                                "User Control"
                            }
                            dd {
                                details {
                                    class: "border border-transparent open:border-main-400 open:rounded-lg open:p-2 open:bg-main-500 mt-1 text-m font-semibold",
                                    summary {
                                        class: " leading-6 select-none",
                                        "Host your OWN Server, or use ours"
                                    }
                                    p {
                                        class: "mt-3 leading-6 text-main-100",
                                        "Whether you prefer our hosted instances, want to run a local server for your LAN, or self-host your own dedicated server, Vauxl gives you the power to choose."
                                    }
                                }
                            }
                        }
                        div {
                            class: "px-4 py-6",
                            dt {
                                class: "font-bold",
                                "Open Source"
                            }
                            dd {
                                details {
                                    class: "border border-transparent open:border-main-400 open:rounded-lg open:p-2 open:bg-main-500 mt-1 text-m font-semibold",
                                    summary {
                                        class: " leading-6 select-none",
                                        "Transparent development, hopefully community-driven"
                                    }
                                    p {
                                        class: "mt-3 leading-6 text-main-100",
                                        "Vauxl is proudly open source. This means our entire codebase is transparently available for anyone to inspect, audit, and contribute to. This commitment fosters trust, as you can verify our claims about privacy and security for yourself. It also encourages community collaboration, allowing developers worldwide to contribute to its evolution, fix bugs, and add new features, ensuring Vauxl remains a vibrant, community-driven platform that truly serves its users."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}