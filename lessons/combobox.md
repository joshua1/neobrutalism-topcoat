# Combobox

1. **`unwrap_or_default()`.** The hidden input renders
   `value=(value.unwrap_or_default())`: the `&str` inside `Some`, or the
   type's `Default` (`""` for strings, `0` for numbers) for `None`. It is
   `value ?? ""` without having to spell out the fallback.

2. **Slicing a string by bytes.** The avatar fallback is
   `&username[..1]`, the first byte as a `&str`. The range counts bytes, not
   characters, and slicing through the middle of a multi-byte character
   (`"émile"[..1]`) panics. It is fine for these ASCII names; for any text,
   take `username.chars().next()` instead.

3. **Nested fixed-size arrays.** `TIMEZONES` is
   `[(&str, [(&str, &str); 6]); 3]`: three regions of exactly six zones. The
   length is part of the type, so a region with five zones would not
   compile. The command example's groups vary in size and use slices
   (`&[Entry]`) instead.

4. **One script, many items.** Every `combobox_item` passes the same
   `SELECT_SCRIPT` constant to `onclick`. A `const` `&str` points at text
   stored once in the binary, so the items share it without allocating. What
   differs per item travels in `data-value`, `data-label` and
   `data-checked`, which the script reads through `this`, the way a closure
   would read its captured variables.
