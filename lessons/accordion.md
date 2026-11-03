# Accordion

1. **`bool` props become boolean attributes.** `accordion_item(disabled: true)`
   renders `inert=(disabled)` as `inert=""`; `false` leaves the attribute out
   entirely. This is what `disabled={flag}` does in JSX, but it is the view
   macro's rule for every `bool` attribute value.

2. **`Option` for attributes that need a value.** `aria-disabled` must say
   `"true"`, not be empty, so the item passes `disabled.then_some("true")`.
   `then_some` turns a `bool` into an `Option`: `Some("true")` when true,
   `None` when false, and a `None` attribute is not rendered. `Option<T>` is
   Rust's `T | undefined`, except the compiler makes you handle the empty case.

3. **Fixed-size arrays of tuples.** The examples keep their data in
   `const BASIC_ITEMS: [(&str, &str, &str); 3]`: an array of exactly three
   tuples, each holding three strings. A tuple is a small anonymous record
   accessed by position, like a TypeScript tuple `[string, string, string]`.

4. **Destructuring in `for`.** `for (value, trigger, content) in BASIC_ITEMS`
   unpacks each tuple into three names, like
   `for (const [value, trigger, content] of items)`. Inside the loop,
   `open=(value == "item-1")` is an ordinary `bool` expression, so only the
   first item renders `open`.
