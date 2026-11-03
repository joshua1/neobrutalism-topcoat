# Empty

1. **Item order doesn't matter.** `empty_media` uses `MEDIA` and
   `EmptyMediaVariant`, which sit further down the file than some code that
   mentions them. Rust resolves functions, types and constants in a module all
   at once, so there's no "used before defined" error as there is with
   `const` in JavaScript.

2. **No HTML escaping by hand.** Octane writes `haven&apos;t`. In Rust it's
   just `"haven't"`, and the view macro escapes text when it renders, so
   `"<b>"` shows up as text instead of becoming markup.

3. **Text and elements together.** `empty_description("Need help? " <a
   href="#">"Contact support"</a>)` passes two child nodes, a text node and
   an element, with no comma between them. Children are a list of nodes, as
   in JSX.
