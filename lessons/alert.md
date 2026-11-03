# Alert

1. **`\` at the end of a line inside a string.** The long class lists end
   lines with `\`. Rust then drops the line break and the next line's
   leading spaces, so `"a \` + newline + `   b"` becomes `"a b"`. Keep a space
   before the `\`, or the two words run together.

2. **`Self` inside an `impl`.** In `impl AlertVariant`, `Self` is another
   name for `AlertVariant`, so `Self::Destructive` is
   `AlertVariant::Destructive`. Renaming the type then means changing one
   line.

3. **A variant named `Default` vs the `Default` trait.**
   `AlertVariant::Default` is just a variant called "Default". The `Default`
   in `#[derive(Default)]` is a trait (an interface) whose
   `AlertVariant::default()` returns whichever variant has `#[default]`. They
   share a word, not a meaning: the trait could just as well return
   `Destructive`.
