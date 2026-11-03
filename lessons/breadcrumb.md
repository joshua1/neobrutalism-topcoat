# Breadcrumb

1. **`Option<T>` replaces `children ?? <ChevronRight />`.**
   `breadcrumb_separator` takes `child: Option<Child<'_>>`: `None` when called
   with no children, `Some(child)` otherwise. Rust has no `null`; a value
   that may be missing says so in its type, and you must handle both cases.

2. **`match` on an `Option`.** `match child { Some(child) => (child), None =>
   icon(..) }` renders the children when there are some and the chevron when
   there are none. The `child` inside `Some(child)` is a new binding holding
   the unwrapped value.

3. **`From<T> for Option<T>`.** The view macro always passes children as a
   plain `Child`. Thanks to `#[into]` and the standard library's conversion
   from `T` into `Option<T>`, the value arrives wrapped in `Some`. Omitted
   children fall back to `#[default]`, which for `Option` is `None`.

4. **Helper functions instead of a `render` prop.** Octane swaps the `<a>`
   for a router `<Link>`. Here `link_attrs(cx, "/", prefetch_mode(cx))`
   returns an `Attributes` value that gives the same `<a>` client-side
   navigation. Examples that call it take `cx: &Cx`, a borrowed request
   context that Topcoat passes in.
