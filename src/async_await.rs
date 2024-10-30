//! Learn
//! - Async Await
//! - Future
//! - Spawn
//! - use_future
//! - use_coroutine
//!
//! Use `use_future`:
//! - When you have a one-time async operation or need to perform async logic upon component mount
//!   or dependency change.
//! - For operations where caching is not needed, such as submitting forms or making a single API
//!   call.
//! - When you want manual control over when the async task runs.
//!
//! Use `use_resource`:
//! - When you need data fetching with caching and automatic reloading based on dependency changes.
//! - For managing data tied to a resource, like fetching a list of items, user profile data, etc.
//! - When you want reactive data fetching that automatically updates when dependencies change.

use crate::{
	async_await::Action::{CloseChat, Idle, SendMessage},
	Route,
};
use async_std::stream::StreamExt;
use dioxus::prelude::*;
use std::{fmt::Display, time::Duration};

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
			hr { class: "my-2 dark:border-gray-700" }
			Example3 {}
			hr { class: "my-2 dark:border-gray-700" }
			Example4 {}
			hr { class: "my-2 dark:border-gray-700" }
			Example5 {}
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

/// Unlike Example2, using `use_future` hook to do background tasks that are tied to the component
/// lifecycle.
fn Example3() -> Element {
	let mut count = use_signal(|| 0);
	let mut running = use_signal(|| false);

	// Using this hook, we can control the Counter timer here.
	let mut f = use_future(move || async move {
		loop {
			if running() {
				count += 1;
			}
			async_std::task::sleep(Duration::from_millis(500)).await; // heavy task
		}
	});

	rsx! {
		div { class: "flex flex-col gap-2",
			h1 { class: "text-2xl font-bold dark:text-gray-200", "Counter: {count}" }
			// NOTE: Once, `f` var is created above for the `use_future` hook, we can use it to pause/resume/cancel the future. This means [Start/Stop] button is no more required once future started. The [Pause]/[Resume] buttons are used to control the future.
			div { class: "flex flex-row gap-2",
				button {
					"data-tooltip-target": "tooltip-start-stop",
					"data-tooltip-placement": "right",
					class: "bg-gradient-to-r from-green-200 to-red-300 hover:bg-gradient-to-l hover:from-green-200 hover:to-red-300 font-bold py-2 px-4 rounded shadow-lg max-w-[160px]",
					onclick: move |_| running.toggle(),
					"Start 🚀/Stop ✋"
				}
				// FIXME: Tooltip is not working.
				// div {
				// 	id: "tooltip-start-stop",
				// 	role: "tooltip",
				// 	class: "absolute z-10 invisible inline-block px-3 py-2 text-sm font-medium text-white bg-gray-900 rounded-lg shadow-sm opacity-0 tooltip dark:bg-gray-700",
				// 	"Tooltip on right"
				// 	div { class: "tooltip-arrow", "data-popper-arrow": "" }
				// }
				button {
					class: "bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded shadow-lg max-w-[150px]",
					onclick: move |_| count.set(0),
					"Reset 🖲️"
				}
			}
			div { class: "flex flex-row gap-2",
				button {
					class: "bg-purple-300 hover:bg-purple-400 font-bold py-2 px-4 rounded shadow-lg max-w-[200px]",
					onclick: move |_| f.restart(),
					"Restart 🔄"
				}
				button {
					class: "bg-yellow-300 hover:bg-yellow-400 font-bold py-2 px-4 rounded shadow-lg max-w-[200px]]",
					onclick: move |_| f.pause(),
					"Pause ⏸️"
				}
				button {
					class: "bg-green-300 hover:bg-green-400 font-bold py-2 px-4 rounded shadow-lg max-w-[200px]",
					onclick: move |_| f.resume(),
					"Resume ▶️"
				}
				button {
					class: "bg-gray-300 hover:bg-gray-400 font-bold py-2 px-4 rounded shadow-lg max-w-[200px]",
					onclick: move |_| f.cancel(),
					"Cancel ❌"
				}
			}
		}
	}
}

/// Using `use_resource` hook to fetch data from API. Similar to `use_future`, but with caching
/// and automatic reloading based on dependency changes.
///
/// Personally, i would prefer this over `use_future` for API calls.
fn Example4() -> Element {
	let mut future = use_resource(|| async move {
		async_std::task::sleep(Duration::from_secs(3)).await;
		"Hello world" // returns hello world from API call.
	});

	rsx! {
		match &*future.read_unchecked() {
			Some(data) => rsx! {
			p { class: "dark:text-gray-200", "{data}" }
			button {
				class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded shadow-lg",
				onclick: move |_| future.restart(),
				"Restart"
			}
		},
			None => rsx! {
			p { class: "dark:text-gray-200", "Loading..." }
		},
		}
	}
}

enum Action {
	Idle,
	SendMessage(String),
	CloseChat,
}

impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Action::Idle => write!(f, "Idle"),
			Action::SendMessage(msg) => write!(f, "Sending Message: {msg}"),
			Action::CloseChat => write!(f, "Closing Chat..."),
		}
	}
}

fn Example5() -> Element {
	let mut current_action = use_signal(|| Idle);

	let chat_client = use_coroutine(move |mut rx: UnboundedReceiver<Action>| async move {
		while let Some(msg) = rx.next().await {
			match msg {
				// Send signal to backend to idle
				Idle => current_action.set(Idle),
				// Send signal to backend to send message
				SendMessage(msg) => {
					current_action.set(SendMessage(msg));
					// Add your backend logic here.
					async_std::task::sleep(Duration::from_secs(3)).await;
					current_action.set(Idle);
				},
				// Send signal to backend to close chat
				CloseChat => current_action.set(CloseChat),
			}
		}
	});

	rsx! {
		div { class: "flex flex-col gap-2 border-2 border-gray-500 rounded-md p-1 shadow-lg max-w-[500px] max-h-[300px]",
			div { class: "relative",
				button {
					class: "bg-red-200 hover:bg-red-300 font-bold text-sm rounded-full items-center justify-center shadow-lg w-6 h-6 absolute top-0 right-0",
					onclick: move |_| chat_client.send(CloseChat),
					"❌"
				}
			}
			div { class: "flex flex-row justify-start w-full h-[100px] border-2 border-gray-100 rounded-md p-2 mt-6",
				p { class: "dark:text-gray-200 text-left text-md text-gray-700", "{current_action}" }
			}
			div { class: "grid grid-cols-12 border-2 border-gray-100 rounded-md",
				div { class: "col-span-10 border-1 rounded-md mr-2 p-2",
					textarea {
						class: " dark:bg-gray-800 w-full focus:outline-none dark:text-gray-200",
						placeholder: "Type your message here...",
					}
				}
				button {
					class: "col-span-2 dark:bg-gray-800 text-blue-600 font-bold px-2 text-5xl rounded",
					onclick: move |_| chat_client.send(SendMessage("Hello 👋".to_string())),
					"➤"
				}
			}
		}
	}
}
