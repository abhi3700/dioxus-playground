---
title: Elements
---

# Elements

## Input

- `oninput` [REQUIRED].
- `value` [OPTIONAL]. When you use the `value` attribute, the input element becomes a controlled component. This means its state is managed by the framework (e.g., Dioxus or React) rather than the DOM itself. Also, it helps to take the initial state of the component as value if no event is triggered.
- Sample code:

    ```rust
    #[component]
    fn InputElement() -> Element {
        let food = "initial value".to_string();

        rsx! {
            input {
                value: "{food}",
                oninput: move |e| food = e.value(),
            }
        }
    }
    ```

- Always keep the text size to be at least 16px i.e. `below-sm:text-base` so that when the user wants to input text, on mobile browsers, don't have to auto-zoom the window. Hence, it leads to a better user experience. Example: It happened with me on a payment page when entering the amount & then going to the next page on submit button click lead to auto-zoomed page which destroyed the UX as the result was exceeding the screen size & the user had to zoom out to see the result. So, it's better to keep the text size to be at least 16px.
  > `below-sm` doesn't exist in Tailwind CSS. So, create like this:

    ```js
    /** @type {import('tailwindcss').Config} */
    module.exports = {
        mode: "all",
        content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
        theme: {
            extend: {
                screens: {
                    "below-sm": { max: "639px" }, // Matches screens smaller than 640px
                },
            },
        },
        plugins: [],
    };
    ```

    > Normally, `sm` is used means it's for small & big screens i.e. min. threshold is 640px. Similarly, for `lg`, `md`.

- `number` type input element is used for entering numbers. But on mobile browsers, it allows to enter alphabets & special characters. Ideally, it should be used for entering numbers & decimal. But the good part is that the value taken into alloted state is always a number like 123 or 123.45. Here is an [example](https://github.com/abhi3700/omnipay-web-rs/blob/9746ec22a5ad6c6221f1246e38e80fcfe1b5582f/src/pages/payment/mod.rs#L201-L286) along with a set of examples.

## Select

- `oninput` [REQUIRED].
- `value` [OPTIONAL]. When you use the `value` attribute, the input element becomes a controlled component. This means its state is managed by the framework (e.g., Dioxus or React) rather than the DOM itself. Also, it helps to take the initial state of the component as value if no event is triggered.
- Sample code:

    ```rust
    # [component]
    fn SelectElement() -> Element {
        let selected_food = use_state(|| "initial_value".to_string());

        rsx! {
            select {
                value: "{selected_food}",
                onchange: move |e| selected_food.set(e.value()),
                option { value: "initial_value", "Initial Value" }
                option { value: "apple", "Apple" }
                option { value: "banana", "Banana" }
            }
        }
    }    
    ```

## Button

## Text

## Image

## Video
