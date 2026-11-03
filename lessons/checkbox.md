# Checkbox

1. **`Option` instead of `undefined`.** `attrs.remove("class")` returns
   `Option<AttributeValue>`: `Some(value)` if the caller passed a class,
   `None` if not. Rust has no `null` or `undefined`, so a missing value is
   always visible in the type, and `class!` skips a `None` entry.

2. **One `attrs`, two destinations.** The class is taken out first and goes
   on the visible `<span>`. Everything else (`id`, `checked`, `@change`)
   goes on the `<input>` with `(attrs)`. Order matters: the class must be
   removed before `attrs` is moved into the input.

3. **Named arguments to nested components.**
   `icon(data: iconify_icon!(..), attrs: attributes! { .. })` calls the
   `icon` component inside our own markup. Arguments are matched by name,
   not position, so their order doesn't matter.
