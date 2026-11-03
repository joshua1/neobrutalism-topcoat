# Toggle Group

1. **`use super::toggle::{...}`.** `super` is the parent module
   (`components`), so this imports from the sibling file `toggle.rs`, like
   `import { toggleVariants } from "./toggle"`. The registry lists `toggle` as
   a dependency so both files are installed together.

2. **Sized integers and `#[default(2)]`.** `spacing: u8` is an unsigned 8-bit
   number (0 to 255); Rust has no single `number` type. `#[default(2)]` gives
   the prop a default other than the type's zero.

3. **`format!` builds a `String`.** `format!("--gap: {spacing}")` interpolates
   the variable named in braces, like a template literal. The result is an
   owned `String` allocated at run time.

4. **Signals and closures in the `custom` example.**
   `signal(cx, || String::from("normal"))` creates browser state; `|| ...` is a
   closure (an arrow function). `@change=$(|e: Event| weight.set(e.target.value))`
   compiles that Rust closure to JavaScript, and `$(weight.get())` re-renders
   when it changes. The component asks for `cx: &Cx` to create the signal.
