//! Sharing state between components
//!
//! This example is showing how to share state between components between parent and child components.
//!
//! Here, the `ThumbnailEditor` is the parent component and the `Thumbnail` and `TitleEditor` are the child components.
//!
//! The `TitleEditor` is using the event from it to make the title change to edited title in the parent component `ThumbnailEditor`.
//! And the `Thumbnail` is using the updated title from the parent component `ThumbnailEditor`.
//!
//! M-1: Lifting approach
//!     - Here, drilling the state down to the child components is required.
//!     - Used when there are a handful of sibling components that need to share state.
//! M-2: Context approach
//!     - Here, the state is provided to the components that need it, via the context i.e. `use_context`. Sharing state is easier.
//!     - Used when there is a deep component tree with many components needing access to the same state.
//!     - More cleaner and easier to maintain.

use crate::Route;
use dioxus::prelude::*;

pub(crate) fn SharingState() -> Element {
    rsx! {
        div { class: "p-2 gap-2",
            div { class: "flex flex-row gap-3 mb-2",
                Link { to: Route::Home {},
                    button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
                        "🔙 🏠"
                    }
                }
                h5 { class: "mb-2 text-xl font-bold tracking-tight text-gray-900 dark:text-white",
                    "Sharing State"
                }
            }
            hr { class: "my-2" }
            Link { to: Route::SharingStateLifting {},
                button { class: "bg-blue-200 hover:bg-blue-300 py-1.5 px-5 rounded-md mr-2 shadow-lg",
                    "Lifting"
                }
            }
            Link { to: Route::SharingStateContext {},
                button { class: "bg-green-200 hover:bg-green-300 py-1.5 px-5 rounded-md shadow-lg",
                    "Context"
                }
            }
        }
    }
}

// ##############################################################################################
// Lifting approach
// ##############################################################################################

pub(crate) fn SharingStateLifting() -> Element {
    rsx! {
        ThumbnailEditor {}
    }
}

/// Parent component for thumbnail and title editor
fn ThumbnailEditor() -> Element {
    let mut title = use_signal(|| "Wolf of Wall Street 🐺".to_string());

    rsx! {
        div { class: "flex flex-col gap-2",
            p { class: "text-2xl font-bold text-gray-700 font-mono", "Thumbnail Editor ✍️" }
            Thumbnail { title }
            TitleEditor { title, oninput: move |e: FormEvent| title.set(e.value()) }
        }
    }
}

/// Thumbnail title component
/// It is using the updated title from the parent component `ThumbnailEditor`.
#[component]
fn Thumbnail(title: String) -> Element {
    rsx! {
        div { class: "border-2 border-gray-700 rounded-md p-2 max-w-[400px]",
            h3 { class: "text-xl font-bold text-blue-900", "Thumbnail" }
            p { class: "text-blue-600", "{title}" }
        }
    }
}

/// Title editor component
/// It is using (calling) the event from here to make the title change to edited title in the parent component `ThumbnailEditor`
#[component]
fn TitleEditor(title: String, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        label { class: "text-gray-500", "Title ✍️" }
        input {
            class: "border-2 border-gray-300 rounded-md p-2 max-w-[400px]",
            value: "{title}",
            oninput: move |e| oninput.call(e),
        }
    }
}

// ##############################################################################################
// Context approach
// ##############################################################################################

pub(crate) fn SharingStateContext() -> Element {
    rsx! {
        ThumbnailEditor2 {}
    }
}

#[derive(Clone)]
struct ThumbnailInfo {
    title: String,
}

/// Parent component for thumbnail and title editor
fn ThumbnailEditor2() -> Element {
    use_context_provider(|| {
        Signal::new(ThumbnailInfo {
            // default value
            title: "Openheimer 🎥💣💥".to_string(),
        })
    });

    rsx! {
        div { class: "flex flex-col gap-2",
            p { class: "text-2xl font-bold text-gray-700 font-mono", "Thumbnail Editor ✍️" }
            Thumbnail2 {}
            TitleEditor2 {}
        }
    }
}

/// Thumbnail title component
/// It is using the updated title from the parent component `ThumbnailEditor`.
fn Thumbnail2() -> Element {
    let thumbnail_info = use_context::<Signal<ThumbnailInfo>>();

    rsx! {
        div { class: "border-2 border-gray-700 rounded-md p-2 max-w-[400px]",
            h3 { class: "text-xl font-bold text-blue-900", "Thumbnail" }
            p { class: "text-blue-600", "{thumbnail_info().title}" }
        }
    }
}

/// Title editor component
/// It is using (calling) the event from here to make the title change to edited title in the parent component `ThumbnailEditor`
#[component]
fn TitleEditor2() -> Element {
    let mut thumbnail_info = use_context::<Signal<ThumbnailInfo>>();

    rsx! {
        label { class: "text-gray-500", "Title ✍️" }
        input {
            class: "border-2 border-gray-300 rounded-md p-2 max-w-[400px]",
            value: "{thumbnail_info().title}",
            oninput: move |e| thumbnail_info.write().title = e.value(),
        }
    }
}
