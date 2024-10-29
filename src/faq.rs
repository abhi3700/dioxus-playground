use dioxus::prelude::*;

use crate::Route;

#[component]
pub(crate) fn Faq() -> Element {
	// State management for accordion sections
	let mut is_open_1 = use_signal(|| false);
	let mut is_open_2 = use_signal(|| false);
	let mut is_open_3 = use_signal(|| false);

	rsx! {
        div {
            id: "accordion-collapse",
            "data-accordion": "collapse",
            class: "p-2",
            div { class: "flex flex-row gap-3 mb-2",
                Link { to: Route::Home {},
                    button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
                        "🔙 🏠"
                    }
                }
                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "FAQs"
                }
            }

            // Accordion Item 1
            h2 { id: "accordion-collapse-heading-1",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 rounded-t-xl focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-1",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-1",
                    onclick: move |_| is_open_1.set(!is_open_1()),
                    span { "What is Flowbite?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: if is_open_1() { "w-3 h-3 shrink-0" } else { "w-3 h-3 shrink-0 rotate-180" },
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5",
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-1",
                class: if is_open_1() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-1",
                div { class: "p-5 border border-b-0 border-gray-200 dark:border-gray-700 dark:bg-gray-900",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Flowbite is an open-source library..."
                    }
                    p { class: "text-gray-500 dark:text-gray-400",
                        "Check out this guide to learn how to "
                        a {
                            href: "/docs/getting-started/introduction/",
                            class: "text-blue-600 dark:text-blue-500 hover:underline",
                            "get started"
                        }
                        " and start developing websites even faster with components on top of Tailwind CSS."
                    }
                }
            }

            // Accordion Item 2
            h2 { id: "accordion-collapse-heading-2",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-b-0 border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-2",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-2",
                    onclick: move |_| is_open_2.set(!is_open_2()),
                    span { "Is there a Figma file available?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: format_args!("w-3 h-3 shrink-0 {}", if is_open_2() { "" } else { "rotate-180" }),
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5",
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-2",
                class: if is_open_2() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-2",
                div { class: "p-5 border border-b-0 border-gray-200 dark:border-gray-700",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Flowbite is first conceptualized and designed..."
                    }
                    p { class: "text-gray-500 dark:text-gray-400",
                        "Check out the "
                        a {
                            href: "https://flowbite.com/figma/",
                            class: "text-blue-600 dark:text-blue-500 hover:underline",
                            "Figma design system"
                        }
                        " based on the utility classes from Tailwind CSS and components from Flowbite."
                    }
                }
            }

            // Accordion Item 3
            h2 { id: "accordion-collapse-heading-3",
                button {
                    "type": "button",
                    class: "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-gray-500 border border-gray-200 focus:ring-4 focus:ring-gray-200 dark:focus:ring-gray-800 dark:border-gray-700 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-800 gap-3",
                    "data-accordion-target": "#accordion-collapse-body-3",
                    "aria-expanded": "true",
                    "aria-controls": "accordion-collapse-body-3",
                    onclick: move |_| is_open_3.set(!is_open_3()),
                    span { "What are the differences between Flowbite and Tailwind UI?" }
                    svg {
                        "data-accordion-icon": "true",
                        class: if is_open_3() { "w-3 h-3 shrink-0" } else { "w-3 h-3 shrink-0 rotate-180" },
                        "aria-hidden": "true",
                        xmlns: "http://www.w3.org/2000/svg",
                        fill: "none",
                        view_box: "0 0 10 6",
                        path {
                            stroke: "currentColor",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            stroke_width: "2",
                            d: "M9 5 5 1 1 5",
                        }
                    }
                }
            }
            div {
                id: "accordion-collapse-body-3",
                class: if is_open_3() { "" } else { "hidden" },
                "aria-labelledby": "accordion-collapse-heading-3",
                div { class: "p-5 border border-t-0 border-gray-200 dark:border-gray-700",
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "The main difference is that..."
                    }
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "However, we actually recommend using both..."
                    }
                    p { class: "mb-2 text-gray-500 dark:text-gray-400",
                        "Learn more about these technologies:"
                    }
                    ul { class: "ps-5 text-gray-500 list-disc dark:text-gray-400",
                        li {
                            a {
                                href: "https://flowbite.com/pro/",
                                class: "text-blue-600 dark:text-blue-500 hover:underline",
                                "Flowbite Pro"
                            }
                        }
                        li {
                            a {
                                href: "https://tailwindui.com/",
                                class: "text-blue-600 dark:text-blue-500 hover:underline",
                                "Tailwind UI"
                            }
                        }
                    }
                }
            }
        }
    }
}
