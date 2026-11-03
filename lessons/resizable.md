# Resizable

1. **Floats are not `Ord`.** `default_size.unwrap_or(1.0).max(0.0)` uses
   `f64::max`, not `std::cmp::max`. An `f64` can be `NaN`, which is neither
   bigger nor smaller than anything, so floats only implement `PartialOrd`,
   and the generic `max` (which needs a total order) refuses them. `f64` has
   its own `max`, `min` and `clamp` that decide what to do with `NaN`.

2. **A method that returns another variant.** `ResizableOrientation::across`
   takes `self` and returns `Self`: a horizontal group's handle is a vertical
   line. The enum is `Copy`, so calling `orientation.across()` doesn't use up
   `orientation`.

3. **Floats in `format!`.** `format!("flex: {size} 1 0px")` prints `50` for
   `50.0`: `Display` for `f64` drops a fractional part of zero, and prints
   `33.5` as `33.5`. Use `{size:.2}` to fix the number of decimals instead.
