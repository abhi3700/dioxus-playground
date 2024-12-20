# Dioxus Playground

My playground for Dioxus.

---

**Activate Tailwind CSS**:

1. `$ npm install -g tailwindcss`.
2. Run the following command in the root of the project to start the tailwind CSS compiler:

```sh
npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

## Web

```sh
LMWR_API_KEY="..." dx serve
```

- Open the browser to <http://localhost:8080> by pressing [o] key in the terminal.

## Desktop

```sh
LMWR_API_KEY="..." dx serve --platform desktop
```

## iOS

```sh
LMWR_API_KEY="..." dx serve --platform ios
```

Open [VSCode extension](https://marketplace.cursorapi.com/items?itemName=DiemasMichiels.emulate)'s emulator from Command palette & choose a device.

## Format

> Do this before committing your changes.

```sh
# 1. Format the Rust, documentation and comments code
cargo fmt
# 2. Format the Dioxus code
dx fmt
```

> For short, you can add a `alias dfcf="dx fmt && cargo fmt"` in your `.zshrc` or `.bashrc` file to use `dfcf` command to format the code.
