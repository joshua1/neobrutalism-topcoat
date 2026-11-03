# Command

1. **`Unescaped::new_unchecked`: opting out of escaping.** Topcoat escapes
   every string it renders, which is right for text but breaks a `<script>`:
   `&&` would arrive as `&amp;&amp;`. The demo wraps its script constant in
   `Unescaped::new_unchecked(..)` to write it verbatim. The `_unchecked`
   suffix is a Rust convention: the compiler can't check it, so you vouch for
   it. Use it only for constants you wrote, never for user input. (Event
   attributes such as `oninput=(COMMAND_SCRIPT)` need nothing: attribute
   escaping is undone by the browser before the JS runs.)

2. **Type aliases.** `type Entry = (IconData, &'static str, Option<&'static str>);`
   gives a long tuple type a short name, like a TypeScript `type`. Unlike a
   tuple struct, it is not a new type: an `Entry` and that tuple are
   interchangeable.

3. **`Clone` but not `Copy`.** `for &(glyph, label, shortcut) in entries`
   failed: the `&(..)` pattern copies each field out of the borrowed tuple,
   and `IconData` holds data that is cloned explicitly, not copied
   implicitly. The fix borrows the tuple (`for (glyph, label, shortcut)`),
   clones just the icon (`glyph.clone()`) and copies the `&str`s out with
   `*label`.

4. **Arrays coerce to slices.** The groups have 4, 6, 4, 5 and 4 entries, so
   they can't share one array type (`[Entry; 4]` and `[Entry; 6]` differ). A
   reference to an array converts to a slice reference, `&[Entry]`, whose
   length is only known at run time, so `[(&str, &[Entry]); 5]` holds them
   all.
