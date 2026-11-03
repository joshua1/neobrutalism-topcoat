# Attachment

1. **Building on another component.** `use super::button::{button, ...}`
   brings in the sibling `button.rs`. Because `attachment.rs` won't compile
   without it, the registry lists `button` as a dependency, and
   `topcoat ui add attachment` installs both.

2. **Forwarding through a wrapped component.** `attachment_action` takes its
   own `attrs`, removes `class`, and hands
   `attributes! { data-slot="attachment-action" class=(class) (attrs) }` to
   `button`. Inside `attributes!`, a spread's keys replace earlier ones, so the
   caller can still override `data-slot`.

3. **`let` before `view!`.** `let class = class!(...)` builds the list first.
   Rust infers its long generic type, so you never write it. The `view!` then
   *moves* `class` into itself. After that line you can't use `class` again,
   because a value has exactly one owner.

4. **`#[default]` on any variant.** `AttachmentState::Done` is the default
   even though it's declared last. The marker goes on the variant you want, not
   the first one.

5. **No tailwind-merge.** Octane's `cn` deletes the button's `size-10` when you
   add `size-6`. `class!` only joins strings, so overrides that clash are
   written `size-6!` to win by importance.
