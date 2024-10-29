#![allow(non_snake_case)]

mod blog;
mod counter;
mod faq;
mod home;
mod sharing_state;

use crate::manganis;
use blog::{Blog, Toi, Tribune};
use counter::Counter;
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use faq::Faq;
use home::Home;
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
}

// Urls are relative to your Cargo.toml file
const STYLE: &str = asset!("assets/tailwind.css");

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        Router::<Route> {}
    }
}
