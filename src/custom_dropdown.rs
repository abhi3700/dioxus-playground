use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
struct DropdownProps {
	options: Vec<String>,
	default_value: String, // Allow default value
}

#[component]
pub(crate) fn Foo() -> Element {
	rsx! {
		div { class: "flex space-x-3 p-3",
			// Light Mode Dropdown
			Dropdown {
				options: vec![
					"Ethereum".to_owned(),
					"Polygon".to_owned(),
					"Sepolia".to_owned(),
				],
				default_value: "Polygon",
			}
		}
	}
}

#[component]
fn Dropdown(props: DropdownProps) -> Element {
	let mut show_menu = use_signal(|| false);
	let mut selected_option = use_signal(|| props.default_value.clone());

	rsx! {
		div {
			class: "relative w-[220px]",
			tabindex: "0",
			onfocusout: move |_| *show_menu.write() = false, // Close when clicking outside
			onclick: move |_| *show_menu.write() = !show_menu(),

			// Main dropdown button styling
			div {
				class: "bg-white border border-gray-300 text-gray-900 text-base rounded-lg focus:ring-blue-500
					focus:border-blue-500 block w-full h-11 px-4 dark:bg-gray-700 dark:border-gray-600 
					dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500 
                    font-sans flex justify-between items-center cursor-pointer",
				"{selected_option}"

				// Arrow icon
				svg {
					class: format_args!("w-3 h-3 text-gray-500 dark:text-gray-300 transition-transform {}", if show_menu() { "rotate-180" } else { "" }),
					width: "12",
					height: "12",
					view_box: "0 0 10 10",
					path {
						d: "M1 2 L5 6 L9 2",
						stroke: "currentColor",
						stroke_width: "2",
						fill: "none",
					}
				}
			}

			// Dropdown menu/tray
			div {
				class: format_args!(
					"absolute left-0 mt-2 w-full bg-white border border-gray-300 rounded-lg shadow-lg
					dark:bg-gray-700 dark:border-gray-600 {}",
					if show_menu() { "block" } else { "hidden" },
				),

				for option in props.options {
					div {
						class: "p-3 cursor-pointer hover:bg-blue-200 dark:hover:bg-blue-500 rounded-sm text-gray-900 dark:text-white",
						onclick: move |_| {
							*selected_option.write() = option.clone();

							// TODO: I need to add some code based on usage.
						},

						span { "{option}" }
						if option == selected_option() {
							i { class: "fa-solid fa-check text-gray-900 dark:text-white ml-2" }
						}

					}
				}
			}
		}
	}
}
