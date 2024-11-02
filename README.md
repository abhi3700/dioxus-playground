# Dioxus Playground

My playground for Dioxus.

## Web

1. Install npm: <https://docs.npmjs.com/downloading-and-installing-node-js-and-npm>
2. Install the tailwind css cli: <https://tailwindcss.com/docs/installation>
3. Run the following command in the root of the project to start the tailwind CSS compiler:

```bash
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

Run the following command in the root of the project to start the Dioxus dev server:

```bash
LMWR_API_KEY="..." dx serve --hot-reload true
```

- Open the browser to <http://localhost:8080> by pressing [o] key in the terminal.

## Desktop

To add desktop support, add "desktop" feature to the existing features in the `Cargo.toml` file for dioxus dependency:

```toml
dioxus = { ... , features = ["desktop"] }
```

Run the following command in the root of the project to start the Dioxus desktop app:

```bash
LMWR_API_KEY="..." dx serve --platform desktop
```

## Format

> Do this before committing your changes.

```sh
# 1. Format the Rust, documentation and comments code
cargo fmt
# 2. Format the Dioxus code
dx fmt
```

> For short, you can add a `alias dfcf="dx fmt && cargo fmt"` in your `.zshrc` or `.bashrc` file to use `dfcf` command to format the code.
