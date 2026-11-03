# Message

1. **`crate::` and `super::` paths.** Inside an app, `crate::components::bubble`
   starts from the crate root. `super::button` (used by other components)
   means "the module next to me". The examples import from both `message` and
   `bubble`; `message.rs` itself doesn't, so it installs without `bubble`.

2. **Private helper components.** The examples' `fallback` is an
   `async fn` without `pub`, so nothing outside the file can call it.
   That's how the examples stand in for `avatar` without exporting a
   lookalike.

3. **Arrays of tuples and destructuring.** `with_actions` stores
   `[("Copy", icon), ...]` and loops with `for (label, data) in actions`. The
   pattern `(label, data)` unpacks each tuple, like
   `for (const [label, data] of actions)`.

4. **`#[key(label)]` on loops that call components.** Each component call
   needs a stable identity. In a loop, the key tells Topcoat which iteration
   is which, the way `key=` does in JSX.
