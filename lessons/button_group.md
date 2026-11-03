# Button Group

1. **One enum, several methods.** `ButtonGroupOrientation` has both
   `classes()` (Tailwind classes) and `as_str()` (the `data-orientation`
   value) in one `impl` block. Each `match` covers both variants, so a third
   orientation must be handled in both places.

2. **A prop default other than the type's `Default`.** The enum's `Default` is
   `Horizontal`, but a separator usually sits in a horizontal group and runs
   vertically, so `button_group_separator` uses
   `#[default(ButtonGroupOrientation::Vertical)]`. The default belongs to the
   prop, not the type.

3. **Enum paths as values.** `ButtonGroupOrientation::Vertical` is an
   ordinary expression, usable anywhere a value is, including inside an
   attribute like `#[default(...)]`.
