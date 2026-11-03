# Chart

1. **`static` vs `const`.** A `const` is pasted into every place that uses
   it, and `&CONST` only lives forever when the value has nothing to drop.
   `ChartSeries` can hold an `IconData`, which does, so a config with icons
   is a `static`: one value at one address for the whole program, and
   `&STATIC` is always `'static`. `const fn with_icon(..)` builds it without
   a `..ChartSeries::new(..)` struct update, which would have to drop the
   unused `icon: None` at compile time (error E0493).

2. **Associated constants.** `Default::default()` can't run in a `const`, so
   `ChartTooltip` also has `pub const DEFAULT: Self`, written inside
   `impl<'a> ChartTooltip<'a>`. Constants built from it,
   `const NO_LABEL: ChartTooltip = ChartTooltip { hide_label: true,
   ..ChartTooltip::DEFAULT };`, are then checked at compile time.

3. **`write!` into a `String`.** `curve_to` appends path commands with
   `write!(path, " L{x:.3},{y:.3}")`, which formats straight into the
   existing string instead of building a temporary one for `push_str`. It
   needs the `std::fmt::Write` trait in scope (`use std::fmt::Write;` inside
   the function), and returns a `Result` that can't fail for a `String`, so
   `let _ =` discards it.

4. **`windows`, `zip` and `fold`.** `points.windows(2)` walks overlapping
   pairs (each segment of a line), `points.iter().zip(data)` walks two
   sequences side by side, and `.fold((0.0, 0.0), |(low, high), value| ..)`
   carries a running minimum and maximum through one pass, like
   `Array.reduce` with a tuple as the accumulator.

5. **Floating-point noise.** `0.7625 / 0.05` is `15.249999999999998`, not
   `15.25`, and `ceil` of a ratio that should be exactly `15` might give `16`.
   `ValueScale::new` rounds the ratio to nine decimals before `ceil` so
   "nice" axis steps stay nice.
