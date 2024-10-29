//! Learn
//! - Async Await
//! - Future
//! - Spawn
//! - use_future
//! - use_coroutine

use crate::Route;
use dioxus::prelude::*;
use std::time::Duration;

pub(crate) fn Async() -> Element {
	rsx! {
		div { class: "p-2 gap-2 dark:bg-gray-800 min-h-screen w-full",
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
			hr { class: "my-2 dark:border-gray-700" }
			Example1 {}
			hr { class: "my-2 dark:border-gray-700" }
			Example2 {}
		}
	}
}

/// In this, on clicking the button, the main thread is blocked for 3 seconds. And then the button
/// comes back.
pub(crate) fn Example1() -> Element {
	let mut is_clicked = use_signal(|| false);

	let handler = move |_| async move {
		is_clicked.set(true);
		async_std::task::sleep(Duration::from_secs(3)).await; // heavy task
		is_clicked.set(false);
	};

	rsx! {
		div {
			if is_clicked() {
				p { class: "dark:text-gray-200", "Something is processing... Main thread is blocked" }
			} else {
				button {
					class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded shadow-lg",
					onclick: handler,
					"Start processing"
				}
			}
		}
	}
}

/// Same as Example1, but we can add multiple concurrent tasks here. Also separate error handling.
/// Here tasks should not be tied to any component lifecycle. For instance, fetching data from API
/// is generally done outside of a component.
pub(crate) fn Example2() -> Element {
	let mut is_clicked = use_signal(|| false);

	let handler = move |_| {
		spawn(async move {
			is_clicked.set(true);
			async_std::task::sleep(Duration::from_secs(3)).await; // heavy task
			is_clicked.set(false);
		});
	};

	rsx! {
		div {
			if is_clicked() {
				p { class: "dark:text-gray-200", "Something is processing... Main thread is blocked" }
			} else {
				button {
					class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded shadow-lg",
					onclick: handler,
					"Start processing"
				}
			}
		}
	}
}
