# Hover card

1. **Integer types.** `delay: u32` is an unsigned 32-bit number: it can't be
   negative, which suits milliseconds. Rust has no single `number`; you pick
   `u32`, `i64`, `f64` and so on, and they don't mix without a conversion.

2. **Literals take their type from context.** In `hover_card(delay: 10, ..)`
   the `10` becomes a `u32` because the prop is `u32`. A prop typed
   `impl Into<Option<u32>>` would not work: many types fit that bound, so the
   literal falls back to `i32`, which has no conversion. That's why the delays
   are plain `u32` with `#[default(600)]` instead of optional.

3. **`&str` is `Copy`.** The sides example uses `(name)` twice: as the
   trigger's label and in the card's sentence. A `&str` is just a pointer and a
   length, so using it again copies it. A `String` would move on first use and
   need `.clone()` for the second.
