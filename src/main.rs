#![allow(non_snake_case)]

mod async_await;
mod blog;
mod counter;
mod faq;
mod home;
mod img_gen;
mod sharing_state;

use crate::manganis;
use async_await::Async;
use blog::{Blog, Toi, Tribune};
use counter::Counter;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use faq::Faq;
use home::Home;
use img_gen::ImgGen;
use sharing_state::{SharingState, SharingStateContext, SharingStateLifting};

/// One global route for the entire App.
#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
	#[route("/")]
	Home {},
	#[route("/blog")]
	Blog {},
	#[route("/blog/tribune")]
	Tribune {},
	#[route("/blog/toi")]
	Toi {},
	#[route("/counter/:id")]
	Counter { id: i32 },
	#[route("/faq")]
	Faq,
	#[route("/sharingstate")]
	SharingState,
	#[route("/sharingstate/lifting")]
	SharingStateLifting,
	#[route("/sharingstate/context")]
	SharingStateContext,
	#[route("/async")]
	Async,
	#[route("/imggen")]
	ImgGen,
}

// Urls are relative to your Cargo.toml file
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
	// Init logger
	dioxus_logger::init(Level::INFO).expect("failed to init logger");
	info!("starting app");
	launch(App);
}

fn App() -> Element {
	rsx! {
		document::Link { rel: "icon", href: FAVICON }
		document::Link { rel: "stylesheet", href: MAIN_CSS }
		document::Link { rel: "stylesheet", href: TAILWIND_CSS }
		Router::<Route> {}
	}
}
