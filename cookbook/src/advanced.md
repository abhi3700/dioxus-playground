# Advanced

- In order to refresh the page, you need to use `window.location.reload()` in the browser console from `web_sys` crate. Sample code:

  ```rust
  use wasm_bindgen::JsValue;

  #[cfg(target_arch = "wasm32")]
  pub(crate) fn reload_window() {
      if let Some(window) = web_sys::window() {
          if let Err(err) = window.location().reload() {
              web_sys::console::error_1(&JsValue::from_str(&format!("Failed to reload: {:?}", err)));
          }
      }
  }

  #[cfg(not(target_arch = "wasm32"))]
  pub(crate) fn reload_window() {
      // TODO: Implement this for desktop
      println!("Reloading window");
  }


  #[component]
  fn Foo() -> Element {
      rsx! {
              button {
                  onclick: |_| {
                  web_sys::window().unwrap().location().reload().unwrap();
              },
              class: "bg-green-600 hover:bg-green-700 text-white text-sm px-4 py-2 rounded-md shadow-md hover:shadow-lg transition-all focus:ring focus:ring-green-300 focus:outline-none",
              "Click here"
          }
      }
  }
  ```

  > This is applicable only for Web App. For desktop app, you have to use some the approach.

- You can avoid connecting to the FE server from client side by using toggle states. Here is an example:

  1. User loads the payment page.
  2. User enters details & clicks on the Submit button.
  3. Now, it communicates directly with the BE server to validate the payment details & get the payment status.
  4. After getting the payment status, it doesn't go to another page, rather remains on the same page & shows the payment status by toggling a variable that rendered a container.

  This way, you can avoid connecting to the FE server from client side. Hence, save cost of the FE server.

- In dioxus, one can't use multi-threaded function like multiple `tokio::spawn` handles joined via `tokio::try_join!`. Because wasm doesn't support atomicity which is required for multi-threading to work. So, all the multi-threaded code should be offloaded to a BE API Server. That's what i have done in a case to fetch the balance & est. fees parallely.

<details><summary><b>Details</b></summary>

 In Rust, WebAssembly (WASM) does not natively support multi-threading in all environments because of limitations in the underlying JavaScript and WebAssembly runtime. Here’s a breakdown of the key reasons why rt-multi-thread (a feature of Tokio and other async runtimes) cannot be used out of the box with WASM when working with libraries like Dioxus:

### 1. WASM Threads Require the wasm32-unknown-unknown Target and the wasm-bindgen Compatibility

- WebAssembly threads rely on Web Workers, which enable running separate threads of execution in the browser.
- However, support for WASM threading in the browser is not fully universal. Even when supported, enabling threading requires:
  - The atomics and shared-memory features to be explicitly turned on during compilation.
  - A compatible host (browser or runtime) with support for these features (e.g., Chrome, Edge, and Firefox, but not Safari as of now).

Without the atomics feature, multi-threading cannot work in WASM.

### 2. Limitations of Tokio and rt-multi-thread in WASM

- `rt-multi-thread` uses operating system-level threads (via `std::thread`) to schedule tasks. WASM, by design, does not have direct access to OS-level threads, as it operates within the browser’s restricted environment.
- Instead, WASM threads are emulated using Web Workers, which require special handling to interact with the main JavaScript event loop. Tokio does not directly support WASM’s threading model.

### 3. Dioxus’ Compatibility with Async in WASM

Dioxus (and other WASM frameworks) generally assume single-threaded execution because:

- WASM’s threading model is still experimental and inconsistent across browsers.
- Dioxus uses async tasks heavily (e.g., for rendering updates and event handling), but it expects these to be run on the main thread in the WASM runtime.
- If multi-threading were used, it would complicate synchronization with the browser’s DOM updates.

</details>
