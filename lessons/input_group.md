# Input Group

1. **`Option<T>` as an optional prop.** `#[default] variant:
   Option<ButtonVariant>` defaults to `None`, Octane's "no variant given".
   `variant.is_some()` asks which case it is, and
   `variant.unwrap_or(ButtonVariant::Neutral)` takes the value or a fallback,
   like `variant ?? "neutral"`.

2. **Nested `use` paths.** `use super::{button::{ButtonVariant, button},
   input::input, textarea::textarea};` pulls items from three sibling modules
   in one statement. The inner braces group items from the same module.

3. **A `const` always has a type.** `const FOCUS_INPUT: &str = "..";` must
   say `&str`, even though the value is obviously a string. Rust infers types
   inside functions, but never for `const`s or function signatures.
