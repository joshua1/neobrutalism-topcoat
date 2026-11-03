# Carousel

1. **`Option::map` turns an optional prop into an optional attribute.**
   `on_select.map(|hook| format!(".. {hook}"))` wraps the hook in the
   carousel's own script only when there is one: `Some` becomes a longer
   `Some`, `None` stays `None`, and `onchange=(on_change)` then leaves the
   attribute out entirely. TypeScript would write `hook && ...`.

2. **Names inside `format!` braces.** `{hook}` in the format string reads the
   variable `hook` directly, the way `${hook}` does in a template literal.
   Only plain names work there; an expression such as `{index + 1}` needs
   `format!("{}", index + 1)`.

3. **JavaScript `${..}` inside a Rust string is just text.** `SHOW_SLIDE`
   holds a JavaScript template literal, `` `Slide ${index+1} of ${count}` ``.
   It is a plain `&str` constant, not a `format!` string, so Rust leaves the
   `${..}` alone and the browser fills it in. Inside `format!` the braces would
   have to be doubled (`{{` and `}}`).
