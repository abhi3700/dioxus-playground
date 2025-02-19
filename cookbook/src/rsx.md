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
    {
        match others().view_mode {
            ViewMode::ByChain => ChainInput { future: wallet_balances_future },
            ViewMode::ByCoin => CoinInput { future: wallet_balances_future },
        }
    }
}

// ✅
div { class: "w-1/3",
    {
        let dropdown = match others().view_mode {
            ViewMode::ByChain => rsx! { ChainInput { future: wallet_balances_future } },
            ViewMode::ByCoin => rsx! { CoinInput { future: wallet_balances_future } },
        };
        dropdown
    }
}
```

## `for`
