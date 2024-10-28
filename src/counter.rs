//! Counter page

use crate::Route;
use dioxus::prelude::*;

#[component]
pub(crate) fn Counter(id: i32) -> Element {
    let mut count = use_signal(|| id);

    rsx! {
        div { class: "flex flex-col gap-2 p-2",
            Link { to: Route::Home {},
                button { class: "bg-gray-200 hover:bg-gray-300 py-1.5 px-5 rounded-md",
                    "🔙 🏠"
                }
            }
            h2 { class: "font-bold", "Hi-fi 🙌 Counter = {count}" }
            div { class: "flex gap-3",
                button {
                    class: "text-white bg-gradient-to-r from-green-400 via-green-500 to-green-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-green-300 dark:focus:ring-green-800 shadow-lg shadow-green-500/50 dark:shadow-lg dark:shadow-green-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                    onclick: move |_| count += 1,
                    "Like 👍"
                }
                // Using tailwind CSS
                button {
                    class: "text-white bg-gradient-to-r from-red-400 via-red-500 to-red-600 hover:bg-gradient-to-br focus:ring-4 focus:outline-none focus:ring-red-300 dark:focus:ring-red-800 shadow-lg shadow-red-500/50 dark:shadow-lg dark:shadow-red-800/80 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                    onclick: move |_| count -= 1,
                    "Dislike 👎"
                }
                button {
                    class: "text-white bg-gradient-to-br from-green-400 to-blue-600 hover:bg-gradient-to-bl focus:ring-4 focus:outline-none focus:ring-green-200 dark:focus:ring-green-800 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2",
                    onclick: move |_| count.set(0),
                    "Reset 🖲️"
                }
            }
        }
        div { class: "flex gap-2 p-2",
            {
                [("Alice", "red"), ("Bob", "blue"), ("Charlie", "green")].iter().map(|&(name, color)| {
                    println!("color: {}", color);
                    rsx! { Workouts { name, color } }
                })
            }
        }
    }
}

// Using struct
#[derive(PartialEq, Props, Clone)]
struct Names {
    name: &'static str,
    color: &'static str,
}

#[component]
fn Workouts(Names { name, color }: Names) -> Element {
    let mut count = use_signal(|| 0);

    let bg_class = match color {
        "red" => "bg-red-300",
        "blue" => "bg-blue-300",
        "green" => "bg-green-300",
        _ => "bg-gray-300", // Fallback color
    };

    rsx! {
        button {
            // NOTE: class & style doesn't work together in terms of expected UI.
            // style: format!("background-color: {}", color),
            class: format!("{} hover:bg-gray-400 py-1.5 px-5 rounded-md hover:text-gray-50", bg_class),
            onclick: move |_| count += 1,
            "Workout by {name} for days: {count} "
        }
    }
}
