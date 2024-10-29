# Development

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to <http://localhost:8080>

## Desktop

To add desktop support, add the following to the `Cargo.toml` file:

```toml
dioxus = { ... , features = ["desktop"] }
```

Run the following command in the root of the project to start the Dioxus desktop app:

```bash
dx serve --platform desktop
```

## Format

> Do this before committing your changes.

```sh
# 1. Format the Rust, documentation and comments code
cargo fmt
# 2. Format the Dioxus code
dx fmt
```
