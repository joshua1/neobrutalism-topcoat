# Sidebar

1. **`macro_rules!` and `concat!`: strings built at compile time.** The toggle
   script appears in three places: the trigger's and rail's `onclick` and the
   shortcut script. A `const` can't be glued to other text in another `const`
   (there is no `+` for `&str` at compile time), but `concat!` joins string
   *literals*. So the script lives in a tiny macro, `toggle_js!()`, that
   expands to a literal, and `concat!("(", toggle_js!(), ")(this)")` builds
   each handler. TypeScript would use a template string at runtime; here the
   result is one `&'static str` baked into the binary.

2. **`?` works on `Option` too.** In `sidebar_state`, the closure
   `|pair| match pair.trim().split_once('=')? { .. }` returns an `Option`.
   `split_once` gives `None` when there is no `=`, and `?` returns that `None`
   from the closure right away, like optional chaining (`a?.b`) that also
   exits the function. `find_map` then keeps the first `Some` across all the
   cookies.

3. **Constants as patterns.** `(SIDEBAR_COOKIE_NAME, "true") => Some(true)`
   matches a tuple whose first item equals the constant. A `const` in a
   pattern compares by value; a lowercase name there would instead bind a new
   variable and match anything.

4. **A plain function can build a view.** `app_sidebar` in the examples is an
   ordinary `fn`, not a `#[component]`. It names the request context with
   `view! { cx => .. }` and returns `.into()` a `Child<'_>`. Returning
   `impl View` would hide the type: callers would only know "some `View`",
   which `view!` cannot interpolate, while `Child` is a concrete type it
   knows how to render.

5. **Randomness without a crate.** The standard library has no random number
   generator, but each `RandomState` (the seed behind `HashMap`) is created
   with random keys. `RandomState::new().build_hasher().finish()` hashes
   nothing and yields a fresh `u64`; `% 40 + 50` maps it to 50..=89, standing
   in for `Math.floor(Math.random() * 40) + 50`.
