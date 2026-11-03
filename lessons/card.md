# Card

1. **`&'static str`: text that lives forever.** `CardSize::as_str` returns
   `"sm"` or `"default"`. String literals are baked into the program binary,
   so a reference to one is valid for the whole run: its lifetime is
   `'static`. Compare `Child<'_>`, which is only borrowed for a while. No
   memory is allocated to hand one back.

2. **`const fn`.** Marking `as_str` as `const fn` lets the compiler run it
   at compile time when its input is known, for example inside another
   `const`. It is an ordinary function otherwise.

3. **Private methods on a public type.** `CardSize` is `pub`, but `as_str`
   and `classes` are not, so only code in `card.rs` can call them. Rust
   privacy is per module (per file here), like a helper you don't `export`.

4. **`use` paths and grouped imports.**
   `use crate::components::card::{card, card_header, CardSize};` is the Rust
   form of `import { Card, CardHeader } from "@/components/ui/card"`.
   `crate::` starts at the root of your own crate, and `{..}` imports several
   names from one module.
