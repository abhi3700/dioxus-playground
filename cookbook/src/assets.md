# Assets

These are extra files (svg, ico, png) that are copied to `public/assets/` folder during build `$ dx serve ..`.

So, must add this line in `Dioxus.toml`:

```diff
[application]
+ asset_dir = "assets"
```
