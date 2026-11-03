# Theme generator

`site/src/theme.rs` turns a cookie into a validated `Theme` and a `Theme`
into CSS; `site/src/app/styling.rs` is the page around it.

1. **The newtype pattern: `struct Hex(String)`.** A tuple struct with one
   field is a new type wrapping an old one. `Hex::parse` is the only way to
   build one (the field is private), and it returns `None` unless the text is
   `#rrggbb`. So anywhere you hold a `Hex`, the compiler has already proved
   it's a valid color. Cookie text can't sneak `red;}` into the CSS. This is
   "parse, don't validate": check once at the edge, then let the type carry
   the guarantee.

2. **`impl Display` makes a type printable.** Implementing
   `fmt::Display` for `Hex` gives it `.to_string()`, lets `format!("{}",
   hex)` work, and lets `view!` render it. TypeScript's equivalent would be a
   `toString()` method.

3. **Function pointers in data: `get: fn(&Theme) -> &Hex`.** `COLOR_SLOTS`
   is a table of 11 color fields, each with a getter and setter written as
   closures (`|t| &t.main`). A closure that captures nothing coerces to a
   plain `fn` pointer, so it can live in a `const`. The form, the cookie
   parser and the cookie writer all loop over this one table instead of
   repeating 11 fields three times.

4. **Pattern tricks for parsing.**
   - `let Some((key, raw)) = pair.split_once('=') else { continue };` binds
     on success and skips the pair otherwise (`let`-`else`).
   - `if let (Some(slot), Some(color)) = (slot, Hex::parse(&raw))` matches
     two `Option`s at once by putting them in a tuple.
   - `match key { "r" => ..., _ => ... }` matches on strings.

5. **A generic function with trait bounds.**
   `fn parse_in<T: FromStr + PartialOrd>(raw: &str, range: RangeInclusive<T>)`
   works for `u8`, `i8` and `u16` alike. `T` must be parseable from text
   (`FromStr`) and comparable (`PartialOrd`), which is exactly what
   `raw.parse().ok().filter(|v| range.contains(v))` needs. `Option::filter`
   turns a `Some` that fails the check into `None`.

6. **`include_str!` embeds a file at compile time.** The registry's
   `themes/neobrutalism.css` is baked into the binary as a `&'static str`.
   `stylesheet()` finds the `:root {` block by byte position (`find`),
   slices around it (`&THEME_CSS[..start]`), and splices in the new block,
   so the output always matches the real theme file.

7. **Unit tests next to the code.** `#[cfg(test)] mod tests` holds plain
   functions marked `#[test]`; `cargo test theme` runs them. They check the
   cookie round trip, that hostile cookie values fall back to defaults, and
   that every preset color is valid. `#[cfg_attr(not(test), allow(dead_code))]`
   silences "unused" for `to_cookie`, which only the tests call.
