# Slider

1. **`Vec<f64>` and float literals.** `value: vec![25.0, 50.0]` builds a
   growable list of 64-bit floats. Rust never converts `25` (an integer) to
   a float for you, so float literals need the `.0`.

2. **Custom defaults: `#[default(100.0)]`.** A parameter can name its own
   fallback expression. It runs only when the caller leaves the prop out,
   which is how `min`, `max` and `step` get Octane's defaults.

3. **Closures.** `let percent = |v: f64| { .. };` is an inline function that
   captures `min` and `max` from around it, like an arrow function. It can be
   called directly, `percent(value[0])`, or passed on:
   `value.last().copied().map_or(0.0, percent)`.

4. **Iterator chains are lazy.** `value.iter().map(..).zip(..).collect()`
   does nothing until `collect` asks for items. `zip` pairs each value with
   its thumb attributes, and `chain(iter::repeat_with(Attributes::new))`
   supplies empty ones forever after the caller's run out. `collect` needs to
   know what to build, hence `let thumbs: Vec<(String, Attributes)>`.

5. **Shadowing.** `let (min, max, step) = (min.to_string(), ..);` makes new
   `String` variables with the old names, hiding the `f64` ones. This is
   allowed and common in Rust: the old values are no longer needed.

6. **`mut` inside a pattern.** `for (thumb_value, mut thumb) in thumbs`
   makes only `thumb` mutable, so its class can be removed with
   `thumb.remove("class")`.
