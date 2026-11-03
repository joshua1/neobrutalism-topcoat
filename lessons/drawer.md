# Drawer

1. **`Option<bool>` for "unset" vs `false`.** `show_swipe_handle` defaults to
   `None`, meaning "decide for me". `show_swipe_handle.unwrap_or(direction ==
   Down)` uses the caller's value if there is one, otherwise the computed
   default. A plain `bool` couldn't tell "not given" from "explicitly false".

2. **`matches!`.** `matches!(self, Self::Up | Self::Down)` is a `bool`: does the
   value fit this pattern? It is a shortcut for a two-armed `match` returning
   `true` / `false`.

3. **`if` is an expression.** `data-modal=(if modal { "true" } else { "false" })`
   picks a value inline, like a ternary. Both branches must have the same type.

4. **Moving in exclusive branches.** `drawer_trigger` renders
   `dialog_trigger(.., attrs: ..)` in one branch and `<button (attrs)>` in the
   other. Each branch moves `attrs` and `child`; that's allowed because only
   one branch ever runs.

5. **Naming a generic return type.** The example helper
   `fn neutral() -> Class<(StaticClass, StaticClass, StaticClass)>` returns
   `button_variants(..)`. A plain function returns classes rather than
   `Attributes`, because `attributes!` needs the request context that
   `#[component]` functions get implicitly.
