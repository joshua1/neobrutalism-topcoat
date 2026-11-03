# Date picker

1. **A fresh identity with `cx.keyed(..)`.** A signal keeps its value across
   re-renders because its identity (the context plus the line that creates
   it) stays the same. The input example wants the opposite when a new date
   is typed: `signal(&cx.keyed(key), || ..)` builds the calendar's signals
   under an identity that includes `key`, so a new date gets new signals,
   which start from their `init` value. The key must implement
   `IdentityKey`; `Option` doesn't, so the date becomes a `String` first with
   `.map(|date| date.to_string()).unwrap_or_default()`.

2. **Splitting on several characters.** `text.split([' ', ','])` splits at
   any space or comma: an array of `char`s is a pattern matching any of them,
   like the regex `/[ ,]/`. Two separators in a row (`", "`) leave an empty
   piece between them, hence `.filter(|word| !word.is_empty())`.

3. **`find` returns an `Option`.** `(1..=12).find(|&month| ..)` stops at the
   first month whose name starts with the typed word, like `Array.find`, but
   gives `Some(month)` or `None` instead of `undefined`. The `?` after it makes
   `parse_typed` return `None` when no month matches.
