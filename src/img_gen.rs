use dioxus::prelude::*;
use dioxus_logger::tracing::info;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::Route;

pub(crate) fn ImgGen() -> Element {
	let mut loading = use_signal(|| false);
	let mut prompt = use_signal(|| "".to_string());
	let mut image = use_signal(|| ImageResponse { data: vec![] });
	let mut error = use_signal(|| None::<String>);

	let mut generate_image = use_resource(move || async move {
		let prompt = prompt.peek().clone();
		if !prompt.is_empty() {
			loading.set(true);
			match generate_images_limewire(prompt).await {
				Ok(response) => {
					loading.set(false);
					image.set(response);
				},
				Err(e) => {
					info!("Error generating images: {:?}", e);
					error.set(Some(e.to_string()));
				},
			}
		}
	});

	rsx! {
		div { class: "p-2 gap-2 dark:bg-gray-800 min-h-screen w-full",
			div { class: "flex flex-row gap-3 mb-2",
				Link { to: Route::Home {},
					button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
						"🔙 🏠"
					}
				}
				h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
					"Image Generator"
				}
			}
			hr { class: "my-2 dark:border-gray-700" }
			input {
				class: "w-full border-2 border-gray-300 rounded-md p-2 mb-2",
				placeholder: "Enter a prompt (e.g. 'A beautiful woman with long blonde hair')",
				oninput: move |e| prompt.set(e.value()),
				value: "{prompt}"
			}
			button {
				class: "bg-purple-200 hover:bg-purple-300 py-1.5 px-5 rounded-md shadow-md",
				onclick: move |_| generate_image.restart(),
				"Generate"
			}
			if loading() {
				LoadingIndicator {}
			}
			if !image().data.is_empty() {
				ImageDisplay { images: image().data }
			}
			if let Some(error) = error() {
				div { class: "text-red-500", "{error}" }
			}
		}
	}
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ImageAssetInfo {
	asset_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Props, Clone)]
struct ImageResponse {
	// other fields ignored.
	data: Vec<ImageAssetInfo>,
}

fn LoadingIndicator() -> Element {
	rsx! {
		div { class: "flex justify-center mt-4",
			div { class: "loader ease-linear rounded-full h-32 w-32 border-8 border-t-8 border-gray-200" }
		}
	}
}

#[component]
fn ImageDisplay(images: Vec<ImageAssetInfo>) -> Element {
	rsx! {
		div { class: "flex flex-wrap gap-4",
			for image in images {
				img { src: "{image.asset_url}", class: "w-48 h-48 object-cover" }
			}
		}
	}
}

// LimeWire API
async fn generate_images_limewire(prompt: String) -> Result<ImageResponse, reqwest::Error> {
	// dotenv::dotenv().ok();
	let lmwr_api_key = std::env::var("LMWR_API_KEY").expect("LMWR_API_KEY must be set");
	let client: reqwest::Client = reqwest::Client::new();

	let payload = json!({
		"prompt": prompt,
		"aspect_ratio": "1:1"
	});

	let response = client
		.post("https://api.limewire.com/api/image/generation")
		.json(&payload)
		.header("Accept", "application/json")
		.header("Content-Type", "application/json")
		.header("Authorization", format!("Bearer {}", lmwr_api_key))
		.header("X-Api-Version", "v1")
		.send()
		.await?;
	let response: ImageResponse = response.json().await?;

	Ok(response)
}
