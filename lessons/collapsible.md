# Collapsible

1. **`mut` only where you change something.** `collapsible_content` forwards
   `attrs` untouched, so its parameter is plain `attrs: Attributes`. Writing
   `mut` there would earn an "unused `mut`" warning: Rust wants mutation to be
   visible and deliberate, unlike `let` vs `const` in TypeScript, where both
   still allow mutating an object's contents.

2. **Attribute lists are maps.** In `attributes! { class="a" class="b" }` the
   second `class` *replaces* the first; there is no `cn` merging. To combine
   a button's classes with your own, build one value:
   `class=(class!(button_variants(..), "w-full"))`.

3. **Plain `&str` constants work as classes.** `const CARD: &str = "..."` is
   enough for the example's stand-in card; `class!(CARD, "max-w-sm")` joins it
   with more classes. `StaticClass` is only needed where a component stores a
   class list for reuse inside other `class!` calls with conditions.
