# Menubar

1. **Items are private to their module by default.** `const ITEM`, `POPUP`
   and the `*_JS` scripts have no `pub`, so they are only visible inside
   `menubar.rs`. That is why `dropdown_menu.rs` and `context_menu.rs` can each
   define their own `ITEM` without a name clash.

2. **Duplication over coupling, on purpose.** The menu parts look alike, but
   menubar does not `use super::dropdown_menu`. Importing it would make the
   registry install both files together and tie their class strings together.
   A little repeated code keeps each component self-contained.

3. **Constants always spell out their type.** `let` can infer types, `const`
   cannot: `const DISMISS_JS: &str` and `const ITEM: StaticClass` both state
   theirs. The type is part of the constant's public shape, so Rust makes you
   write it.
