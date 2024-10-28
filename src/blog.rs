//! Blog page

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Blog() -> Element {
	rsx! {
		div { style: "display: flex; gap: 5px; padding: 2px; flex-direction: column",
			Link { to: Route::Home {},
				button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
					"🔙 🏠"
				}
			}
			nav { style: "display: flex; gap: 20px; padding: 10px; background-color: #EEC995;",
				Link { class: "text-blue-800 hover:underline", to: Route::Tribune {}, "Tribune" }
				Link { class: "text-blue-800 hover:underline", to: Route::Toi {}, "Times of India" }
			}
		}
		h2 { "Blog" }
		hr { style: "border: 1px solid #1F2937" }
		h3 { style: "color: #1F2937; font-weight: bold", "Categories" }
		{vec!["Entertainment", "Sports", "Politics", "Business", "Technology", "Science", ].iter().map(|cat| rsx! {
			li { class: "text-blue-800 hover:underline p-2", "{cat}" }
		})}
	}
}

use crate::manganis;
static ASSET_PATH: &str = asset!("assets/tribune-logo.jpg");

#[component]
pub(crate) fn Tribune() -> Element {
	rsx! {
		Link { to: Route::Blog {},
			button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md", "🔙" }
		}
		h1 { "📰 The Tribune" }
		img { src: ASSET_PATH, alt: "The Times of India Logo" }
	}
}

/// Shows State management with Conditional Rendering
#[component]
pub(crate) fn Toi() -> Element {
	println!("Toi rendered");

	// State management using a signal to toggle the visibility of the content
	let mut expanded = use_signal(|| false);

	rsx! {
		Link { to: Route::Blog {},
			button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-6 rounded-md", "🔙" }
		}
		if true {
			h1 { "📰 The Times of India" }
		}
		button {
			class: "bg-blue-200 hover:bg-blue-300 py-1.5 px-5 rounded-md",
			onclick: move |_| {
				expanded.set(!expanded());
				dbg!("{:?}", expanded());
				needs_update();
			},
			if expanded() {
				"Collapse 🗞️"
			} else {
				"Expand 🗞️"
			}
		}
		if expanded() {
			img {
				src: "https://c8.alamy.com/comp/ERYFEF/times-of-india-building-bombay-mumbai-maharashtra-india-asia-old-vintage-ERYFEF.jpg",
				alt: "The Times of India Building, Bombay"
			}
		}
	}
}
