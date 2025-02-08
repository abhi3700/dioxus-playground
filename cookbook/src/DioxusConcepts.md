# Dioxus Concepts

## Components

- Use of `#[component]` attribute is used to define a component. It's not necessary as such for functions that don't have any props. However, it's a good practice to use it. Make sure to mandatorily use `#[component]` attribute for functions that have props though.
- Using `#[derive(Props)]` attribute prevents from using `.read()` on the type. Instead of using `payload.read().amount`, use `payload.amount` where:

```rust
let payload: Signal<Payload> = use_signal(|| Payload { from: "USD".to_string(), to: "EUR".to_string(), amount: 100 });
```

- This is a good practice as it makes the code more readable and easier to understand.
- Props are immutable.

## Signals

- If you have multiple booleans, then don't define separate signals for them. Instead, define a single signal with a struct like this:

```rust
#[derive(Props, PartialEq, Clone)]
struct BoolFlags {
    is_loading: bool,
    is_error: bool,
    is_success: bool,
}

fn Parent() -> Element {
    let bool_flags: Signal<BoolFlags> = use_context_provider(|| Signal::new(BoolFlags { is_loading: false, is_error: false, is_success: false }));

    rsx! {
        Child1()
        Child2()
    }
}
```

Inside the related components, where the values needs to be toggled/updated, use the `use_context` to get the signal and update the values like this:

```rust

fn Child1() -> Element {
    let mut bool_flags = use_context::<Signal<BoolFlags>>();

    rsx! {
        button {
            onclick: move |_| bool_flags.write().is_loading = true,
            "Toggle Loading"
        }
    }
}

fn Child2() -> Element {
    let mut bool_flags = use_context::<Signal<BoolFlags>>();

    rsx! {
        button {
            onclick: move |_| bool_flags.write().is_error = true,
            "Toggle Error"
        }
    }
}
```

- This is because a signal is a single source of truth. That means it can't update so many boolean values because they are identified by types. And all have bool types. Hence, better you create a struct with multiple named bool flags to avoid confusion.
- When updating the Props' fields (≥ 2), then prefer 1 over 2 below:
  
✅:

```rust
let mut current_history = pay_history.write();                        
current_history.receipts.extend(next_page.receipts);
current_history.has_next = next_page.has_next;
```

> Reason:
>
> - Because using 2 `write()` for the same Props struct would leave it hanging for panic especially used with `unchecked`. Best way is to use `RefCell` to lock the struct when writtten.

❌:

```rust
pay_history.write().receipts.extend(receipts);
pay_history.write().has_next = has_next;
```
