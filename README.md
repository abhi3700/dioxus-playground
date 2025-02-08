# Dioxus Playground

My playground for Dioxus.

## [Cookbook](./cookbook/)

--- 🧑‍💻 ---

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

## Deploy

1. Bundle your project for Web (say) platform via `$ LMWR_API_KEY="..." dx bundle -r --platform web`.
2. `$ netlify status` or `$ netlify login` to check your status or login.
3. Go to the output directory i.e. `$ cd target/dx/playg/release/web/public`.
4. Create site on Netlify via `$ netlify sites:create` if you want to go w/o CI/CD pipeline or share github public repo url.
5. `netlify deploy` to deploy your project. Given the current directory via `.`.
6. `netlify deploy --prod` to deploy your project to production.
7. To get the site url, run `$ netlify status`.
