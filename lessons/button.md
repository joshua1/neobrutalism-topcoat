# Button

1. **Enums replace `cva` variants.** `variant` was a string like `"reverse"`; in
   Rust it is `enum ButtonVariant { Default, NoShadow, Neutral, Reverse }`. A
   typo such as `ButtonVariant::Revers` is a compile error, not a silently
   unstyled button. `#[default]` marks the variant used when none is given.

2. **`#[derive(...)]` generates code for you.** `Clone, Copy` let the value be
   copied freely, like a number. `Debug` makes it printable, `Default` uses the
   `#[default]` variant, and `PartialEq` allows `==`.

3. **`impl` + `match` maps each variant to classes.** An `impl` block attaches
   methods to a type. `match` must cover every variant: add a fifth variant and
   the compiler points at `classes()` until you style it, something `cva`
   can't guarantee.

4. **`const BASE: StaticClass = class!(...)`** is a value fixed at compile
   time: the base string you passed first to `cva(...)`.

5. **`#[component] pub async fn button(...)`.** An attribute macro turns the
   function into a component. Each parameter becomes a prop, `#[default]` makes
   a prop optional, and `pub` exports it from the module.

6. **`mut attrs` and `attrs.remove("class")`.** A variable can only be changed
   if it is marked `mut`. We take the caller's `class` out of `attrs`, append
   it after ours, and spread the rest with `(attrs)`. This is `{...props}`
   plus `cn(..., klass)`.

7. **`Result<impl View>` and `Ok(...)`.** Rust has no exceptions. A function
   that can fail returns `Result`, which is either `Ok(value)` or `Err(error)`.
   `impl View` means "some type that implements View, unnamed", similar to
   returning JSX.

8. **`Child<'_>` is a first look at lifetimes.** `'_` means the children are
   borrowed from the caller, not copied, and the compiler checks the button
   doesn't hold them longer than the caller does.

9. **`#[cfg(test)]` and `[dev-dependencies]`.** Code behind `#[cfg(test)]` only
   compiles for tests. `topcoat` is a dev-dependency here, so apps that depend
   on this crate download nothing extra; they only copy source files.

10. **`bool::then_some` turns a condition into an `Option`.**
    `(!attrs.contains_key("data-slot")).then_some("button")` is
    `Some("button")` when the caller didn't pass a `data-slot`, else `None`.
    `view!` renders `None` as "no attribute", so a wrapper can replace the
    slot without the attribute appearing twice. It's the Rust spelling of
    `attrs["data-slot"] ? undefined : "button"`.
