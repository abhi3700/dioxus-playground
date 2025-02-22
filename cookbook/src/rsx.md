# RSX

Mixing Rust 🦀 with Dioxus element, is what **RSX** is.

## `if-else`

## `match`

- Unlike `if-else`, can't directly return values as rsx!. This is because `match` expression returns values. So, need to wrap with `rsx!`.

> - **Rust’s ownership & borrowing**: match expressions return values, so assigning the rsx! output to a variable helps Dioxus understand the rendering logic.
> - **Dioxus rendering**: rsx! doesn’t directly accept match because it expects a single JSX-like node at a time.

```rust
// ❌
div { class: "w-1/3",
    match others().view_mode {
        ViewMode::ByChain => ChainInput { future: wallet_balances_future },
        ViewMode::ByCoin => CoinInput { future: wallet_balances_future },
    }
}
```

Here, alongwith the codeblock, wrap with `rsx!`:

```diff
// ✅
div { class: "w-1/3",
+    {
        match others().view_mode {
            ViewMode::ByChain => rsx! { ChainInput { future: wallet_balances_future } },
            ViewMode::ByCoin => rsx! { CoinInput { future: wallet_balances_future } },
        }
+    }
}
```

## `for`

## `block`

Place code inside a dioxus element tag like this:

```rust
     p { class: "font-bold tracking-wide text-gray-800 dark:text-gray-100",
     // NOTE: Block introduced & also returned element tag wrapped with rsx!.
      {
          let parts: Vec<&str> = balances.total_usd.split('.').collect();
          rsx! {
       span { class: "text-4xl below-sm:text-3xl", "$ {parts.get(0).unwrap_or(&\"0\")}" }
       span { class: "text-2xl below-sm:text-xl", ".{parts.get(1).unwrap_or(&\"00\")}" }
      }
      }
     }

```
