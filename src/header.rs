use dioxus::prelude::*;

use gloo_timers::future::sleep;
use std::time::Duration;

const ICON: Asset = asset!("/assets/vauxl_icon03.png");

#[component]
pub fn Header() -> Element {
    let mut is_main_menu_open = use_signal(|| false);
    let mut close_task_handle: Signal<Option<Task>> = use_signal(|| None);

    rsx! {
        header {
            class: "bg-blue-040 text-white shadow-md font-main",
            nav {
                class: "mx-auto flex max-w-7xl items-center justify-between px-4 lg:px-8",
                aria_label: "Global",
                div{
                    class: "flex lg:flex-1",
                    a {
                        class: "mr-2 my-3",
                        href: "/",
                        span{
                            class: "sr-only",
                            "VauxlNet",
                        },
                        img {
                        class: "mask-origin-content h-8 w-auto",
                        src: ICON,
                        },
                    },
                    a {
                        class: "text-xl font-bold my-3",
                        href: "/",
                        "VauxlNet",
                    },
                },
                div {
                    class: "flex lg:hidden",
                    button {
                        type: "button",
                        class: "-m-2.5 inline-flex items-center justify-center rounded-md p-2.5 text-gray-300",
                        span {
                            class: "sr-only",
                            "OpenMainMenu",
                        },
                        svg {
                            class:"size-6 aria-hidden data-slot",
                            fill:"none",
                            view_box:"0 0 24 24",
                            stroke_width:"1.5",
                            stroke:"currentColor",
                            path {
                                stroke_linecap:"round",
                                stroke_linejoin:"round",
                                d:"M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5",
                            },
                        },
                    },
                },
                div {
                    class: "hidden lg:flex lg:gap-x-12",
                    a {
                        class: "rounded-lg px-3 py-2 mx-6 text-white font-medium hover:bg-blue-030 hover:text-gray-100 transition duration-300",
                        href: "/team",
                        "Team",
                    },
                    a {
                        class: "rounded-lg px-3 py-2 mx-6 text-white font-medium hover:bg-blue-030 hover:text-gray-100 transition duration-300",
                        href: "/projects",
                        "Projects",
                    },
                    a {
                        class: "rounded-lg px-3 py-2 mx-6 text-white font-medium hover:bg-blue-030 hover:text-gray-100 transition duration-300",
                        href: "/dashboard",
                        "Dashboard",
                    },
                    div {
                        class: "relative",
                        onmouseover: move |_| {
                            if let Some(handle) = close_task_handle.write().take() {
                                handle.cancel();
                            }
                            is_main_menu_open.set(true);
                        },
                        onmouseout: move |_| {
                            let handle = spawn(async move {
                                sleep(Duration::from_millis(200)).await;
                                is_main_menu_open.set(false);
                            });
                            close_task_handle.set(Some(handle));
                        },
                        a {
                            class: "flex items-center gap-x-1 rounded-lg px-3 py-2 mx-6 font-medium hover:bg-blue-030 hover:text-gray-100 transition duration-300 aria-expanded-f",
                            href: "/contact",
                            "Contact",
                            svg {
                                class: "size-5 flex-none text-gray-400 aria-hidden data-slot",
                                view_box: "0 0 20 20",
                                fill: "currentColor",
                                path {
                                    fill_rule: "evenodd",
                                    d:"M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z",
                                    clip_rule: "evenodd",
                                },
                            },
                        },
                        if is_main_menu_open() {
                            div {
                                class: "absolute top-full -left-8 z-10 mt-3 w-screen max-w-md overflow-hidden rounded-3xl bg-blue-200 shadow-lg ring-1 ring-gray-900/5",
                                div {
                                    class: "p-4",
                                    div {
                                        class: "group relative flex items-center gap-x-6 rounded-lg p-4 text-sm/6 hover:bg-blue-030",
                                        div {
                                            class: "flex-auto",
                                            a {
                                                class: "block rounded-lg text-blue-040 font-medium hover:text-gray-100 aria-expanded-true",
                                                href: "/contact/report",
                                                "Report"
                                                span {
                                                    class: "absolute inset-0",
                                                }
                                            },
                                            p {
                                                class: "block font-medium text-gray-400",
                                                "Report a security Thingy (Please correct me)"
                                            },
                                        },
                                    },
                                    div {
                                        class: "group relative flex items-center gap-x-6 rounded-lg p-4 text-sm/6 hover:bg-blue-030",
                                        div {
                                            class: "flex-auto",
                                            a {
                                                class: "block rounded-lg text-blue-040 font-bold hover:text-white aria-expanded-true",
                                                href: "/contact/info",
                                                "Info"
                                                span {
                                                    class: "absolute inset-0",
                                                }
                                            },
                                            p {
                                                class: "block font-medium text-gray-400",
                                                "get Information about anything"
                                            },
                                        },
                                    },
                                },
                            },
                        }
                    },
                    div {
                        class: "hidden lg:flex lg:flex-1 lg:justify-end",
                        a {
                            class: "rounded-lg px-3 py-2 mx-6 text-white font-bold hover:bg-blue-030 hover:text-gray-100 transition duration-300",
                            href: "/login",
                            "LogIn",
                            span {
                                class: "aria-hidden",
                                "&rarr;",
                            },
                        },
                    },
                },
            },
            //TODO: working opening and closing of nav bar window
            div {
                class: "lg:hidden",
                role: "dialog",
                aria_modal: "true",
                div {
                    class: "fixed inset-0 z-10",
                },
                div {
                    class: "fixed inset-y-0 right-0 z-10 w-full overflow-y-auto bg-blue-040 text-white px-6 py-6 sm:max-w-sm sm:ring-1 sm:ring-gray-900/10",
                    div{
                        class: "flex lg:flex-1",
                        a {
                            class: "mr-2 my-3",
                            href: "/",
                            span{
                                class: "sr-only",
                                "VauxlNet",
                            },
                            img {class: "mask-origin-content h-8 w-auto",src: ICON,
                            },
                        },
                        a {
                            class: "text-xl font-bold my-3",
                            href: "/",
                            "VauxlNet",
                        },
                    },
                    button {
                        type:"button",
                        class: "-m-2.5 rounded-md p-2.5 text-gray-300",
                        span {
                            class: "sr-only",
                            "CloseMenu",
                        }
                        svg {
                            class: "size-6 aria-hidden data-slot",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "1.5",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "M6 18 18 6M6 6l12 12",
                            },
                        },
                    },
                },
                div {
                    class: "mt-6 flow-root",
                    div {
                        class: "-my-6 divide-y divide-gray-500/10",
                        div {
                            class: "space-y-2 py-6",
                            a {
                                class: "block rounded-lg px-3 py-2 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                href: "/team",
                                "Team"
                            },
                            a {
                                class: "block rounded-lg px-3 py-2 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                href: "/project",
                                "Projects"
                            },
                            a {
                                class: "block rounded-lg px-3 py-2 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                href: "/dashboard",
                                "Dashboard"
                            },
                            div {
                                class: "-mx-3",
                                button {
                                    type: "button",
                                    class: "flex w-full items-center justify-between aria-controls-dis-1 aria-expanded-f rounded-lg py-2 pr-3.5 pl-3 text-white font-bold hover:bg-blue-030 hover:text-gray-100",
                                    "Contact"
                                    svg {
                                        class: "size-5 flex-none text-gray-400 aria-hidden data-slot",
                                        view_box: "0 0 20 20",
                                        fill: "currentColor",
                                        path {
                                            fill_rule: "evenodd",
                                            d:"M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z",
                                            clip_rule: "evenodd",
                                        },
                                    },
                                },
                                div {
                                    class: "mt-2 space-y-2",
                                    id: "disclosure-1",
                                    a {
                                        class: "block rounded-lg py-2 pr-3 pl-6 text-white font-bold hover:bg-blue-030 hover:text-gray-100",
                                        href: "/contact",
                                        "Contact"
                                    },
                                    div {
                                        class: "ml-2",
                                        a {
                                        class: "block rounded-lg py-2 pr-3 pl-6 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                        href: "/contact/report",
                                        "Report"
                                        },
                                        a {
                                        class: "block rounded-lg py-2 pr-3 pl-6 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                        href: "/contact/info",
                                        "Information"
                                        },
                                    },
                                },
                            },
                        },
                        div {
                            class: "py-6",
                            a {
                                class: "-mx-3 block rounded-lg px-3 py-2.5 text-white font-medium hover:bg-blue-030 hover:text-gray-100",
                                href: "/login",
                                "LogIn"
                            },
                        }
                    },
                },
            },
        },
    }
}