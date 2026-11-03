# Toggle

1. **A `pub` method another file can call.** `ToggleKind::input_type` is
   `pub fn`, unlike the private `classes()` helpers, because `toggle_group.rs`
   needs it too. Visibility is set per item, not per type.

2. **Tuple structs (newtypes).** `PromotedStr(&"checkbox")` builds a struct
   whose single field has no name, `struct PromotedStr(pub &'static &'static str)`.
   Wrapping a value in its own type tells Topcoat this string is a static
   literal it can emit without copying.

3. **Arms may return the same value.** `Default` and `Outline` both map to
   `"bg-secondary-background text-foreground"`, as in Octane. They could be
   merged with an or-pattern, `Self::Default | Self::Outline => ...`; separate
   arms keep room to restyle one later.
