# Calendar

1. **`div_euclid` and `rem_euclid` for negative numbers.** `Date::from_days`
   counts days from 1970, so earlier dates are negative. Rust's `/` and `%`
   round toward zero, as JavaScript's do: `-1 / 7` is `0` and `-1 % 7` is
   `-1`. `(-1i64).div_euclid(7)` is `-1` and `(-1i64).rem_euclid(7)` is `6`,
   which is the answer date math wants ("the day before a Thursday is a
   Wednesday", not "weekday -1").

2. **`#[derive(PartialOrd, Ord)]` on a struct.** The derived comparison looks
   at the fields in declaration order, like comparing tuples. `Date` declares
   `year`, then `month`, then `day`, so `<` and `>` are chronological, and
   `min`, `max` and `clamp` work on dates with no hand-written code. Swap the
   fields and the comparison would quietly change meaning.

3. **`Display` and `FromStr` give you `to_string()` and `parse()`.**
   Implementing the standard `fmt::Display` trait lets a `Date` go anywhere a
   string is formatted: `date.to_string()`, `format!("{date}")`. Implementing
   `FromStr` (with `type Err` naming its error) lets any string become one:
   `"2025-06-01".parse::<Date>()`. Each trait you implement unlocks methods
   that already exist for other types, similar to giving a JavaScript class a
   `toString()`.

4. **`?` with your own error: `ok_or` and `map_err`.** `from_str` returns
   `Result<Date, ParseDateError>`. A missing part is an `Option`, so
   `.ok_or(ParseDateError)?` turns `None` into that error. A bad number is a
   `ParseIntError`, so `.map_err(|_| ParseDateError)?` swaps the error type.
   `?` then returns early on either, like a `throw` that the signature
   announces.

5. **`as_deref`: from `Option<String>` to `Option<&str>`.** A signal holds an
   `Option<String>`, and `parse` wants a `&str`. `value.as_deref()` borrows the
   string inside without moving it out, so
   `signal.get().as_deref().and_then(|text| text.parse().ok())` reads "if
   there is text, try to parse it".
