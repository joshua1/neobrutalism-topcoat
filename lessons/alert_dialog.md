# Alert dialog

1. **`use super::...` imports a sibling module.** Every component file is a
   module inside `components`. `super` is that parent, so
   `use super::dialog::{dialog, dialog_close}` reaches the dialog file next door
   (like `import { dialog } from "./dialog"`). This is why `registry.toml` lists
   `dialog` and `button` as dependencies: `topcoat ui add` must copy them too.

2. **Components wrap components.** `alert_dialog` calls `dialog(open: open, ..)`
   and forwards its own `open`. The value *moves* into the inner call, so there
   is no copy and no `clone()`; `alert_dialog` never uses `open` again.

3. **Later entries win in `attributes!`.**
   `attributes! { data-slot="alert-dialog" role="alertdialog" (attrs) }` puts the
   defaults first and spreads the caller's `attrs` after them, so a caller's
   `role` replaces the default, like `{ role: "alertdialog", ...props }`.

4. **A default per component.** `alert_dialog_cancel` declares
   `#[default(ButtonVariant::Neutral)] variant`, while `alert_dialog_action`
   uses the plain `#[default]` (`ButtonVariant::Default`). Same type, different
   defaults, the way Octane's two functions had `variant = "neutral"` and
   `variant = "default"`.

5. **Enums to strings with `as_str`.** `AlertDialogSize::as_str(self) ->
   &'static str` maps each variant to its `data-size` value. Rust enums are not
   strings underneath, so the mapping is written out, and `match` checks it
   covers every size.
