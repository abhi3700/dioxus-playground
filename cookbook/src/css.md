# CSS

In order to add CSS designs to app, you need to follow either of these approaches.

> NOTE: Please note that a `assets/tailwind.css` file is automatically generated on running this command on a separate terminal:

```sh
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Actually the files in content field in `tailwind.config.js` file is tracked when saving any rust, html files:

```js
module.exports = {
    mode: "all",
    content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
```

So, just in case, if you have related lib outside `src/` or `dist/`, then you have to add that directory into the array like this:

```diff
...
    content: [
+        "./common/**/*.{rs,html,css}",
        "./src/**/*.{rs,html,css}",
        "./dist/**/*.html",
    ],
...
```

---

Don't forget to add this line in `Dioxus.toml`:

```diff
[application]
+ asset_dir = "assets"
```

## A. Add CSS to `input.css`

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

// ...

.disable-double-tap-zoom {
    touch-action: manipulation;
}
```

And then use like this:

```rust
#[components]
fn foo() -> Element {
    rsx! {
        div {
            class: "disable-double-tap-zoom",
        }
    }
}
```

## B. Add tailwind css to the `.rust`, `.html` files directly

```rust
#[components]
fn foo() -> Element {
    rsx! {
        div {
            class: "text-blue-100",
        }
    }
}
```
