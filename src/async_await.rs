//! Learn
//! - Async Await
//! - Future
//! - Spawn
//! - use_future
//! - use_coroutine

use crate::Route;
use dioxus::prelude::*;

pub(crate) fn Async() -> Element {
    rsx! {
        div { class: "p-2 gap-2",
            div { class: "flex flex-row gap-3 mb-2",
                Link { to: Route::Home {},
                    button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
                        "🔙 🏠"
                    }
                }
                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Async Await"
                }
            }
            hr { class: "my-2" }
            Example1 {}
        }
    }
}

pub(crate) fn Example1() -> Element {
    rsx! {
        div { "Example 1" }
    }
}
