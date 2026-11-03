# Select

1. **`cx: &Cx`, the request context.** A component that declares `cx: &Cx`
   gets the current request's context. `select` needs it to render the
   checkmark icon into an SVG string with `.render(cx)`. Callers never pass it;
   Topcoat supplies it.

2. **Building a `String` piece by piece.** `String::from(..)` makes an owned,
   growable string. `push_str` appends a `&str`, `push` appends one `char`,
   and `for char in svg.chars()` walks it character by character. `match` on
   character literals (`'%'`, `'"'`) picks the escape.

3. **Raw strings: `r#"..."#`.** Inside `r#"` and `"#`, quotes and backslashes
   are literal, so SVG markup full of `"` needs no escaping.

4. **`Option<&str>` with `#[into]` and `if let`.** `placeholder` is
   `Option<&str>`: `Some("Select a fruit")` or `None`. `#[into]` lets callers
   write `placeholder: "Select a fruit"`, converting to `Some(..)` for them.
   In the view, `if let Some(placeholder) = placeholder { ... }` renders the
   option only when there is one and unwraps the text in the same step.
