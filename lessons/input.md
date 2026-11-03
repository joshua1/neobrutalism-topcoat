# Input

1. **A component without `child` takes no children.** `input` only has an
   `attrs` parameter, because `<input>` is a void element. Passing children
   to it is a compile error, not something silently ignored.

2. **Long strings across lines with `\`.** A `\` at the end of a line inside
   a string literal joins the next line and drops its leading spaces. That is
   how the long Tailwind class list is wrapped without adding newlines to it.
   Keep a space before each `\` so the class names stay separated.

3. **Keywords inside macros.** `type` is a Rust keyword, yet
   `attributes! { type="email" }` is fine. A macro reads raw tokens and
   decides their meaning itself, so `type=` and `for=` mean HTML attributes
   there.
