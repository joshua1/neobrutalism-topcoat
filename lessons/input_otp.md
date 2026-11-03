# Input OTP

1. **Escapes vs raw strings.** `"^\\d+$"` needs `\\` because `\` starts an
   escape in a normal string literal. A raw string, `r"^\d+$"`, would keep
   backslashes as written, which is handy for regular expressions.

2. **`char` is not a one-letter `String`.** `value.chars()` yields `char`s,
   single Unicode characters written in single quotes (`'a'`, `'\''`). The
   `match c { '\'' | '\\' => .., _ => .. }` arm uses `|` to match either of
   two characters.

3. **`collect` builds a `String`, too.** `.take(max_length).enumerate()
   .map(|(i, c)| format!(..)).collect()` joins the pieces into one `String`,
   because the binding is typed `let chars: String`. `enumerate` pairs each
   item with its index, a `usize`.

4. **`bool` to `Option`.** `(!chars.is_empty()).then_some(chars)` is
   `Some(chars)` when true and `None` when false. A `None` attribute value is
   left out of the HTML. `pattern.is_none_or(|p| ..)` is `true` for `None` or
   when the closure says so.

5. **Ranges.** `for index in 0..6` counts 0 to 5: the end of `a..b` is
   excluded. Write `0..=6` to include it.
