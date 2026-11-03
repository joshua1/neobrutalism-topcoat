# Message scroller

1. **Raw strings: `r#"..."#`.** The inline JavaScript is full of quotes. A raw
   string ignores escapes and ends only at `"#`, so the JS is pasted
   verbatim, like a template literal with no interpolation.

2. **`match` in an attribute list.** The viewport removes `aria-label` from
   `attrs` and matches the `Option`: `Some(label) => aria-label=(label)`,
   `None => aria-label="Messages"`. That gives a default without rendering the
   attribute twice.

3. **Shards and signals.** `add_messages_chat` is a `#[shard]`. Clicking "Add
   message" bumps the `count` signal in the browser, and the server re-renders
   just that shard. The browser sends `count`, so it's user input:
   `.clamp(INITIAL, 100)` keeps it in range.

4. **Iterator chains.** `(0..shown).map(..)` and `.into_iter().enumerate()`
   build lazy pipelines. Nothing runs until the `for` loop pulls items, like
   a JavaScript generator rather than `Array.map`.

5. **Patterns can collide with names.** `Some(&message)` failed to compile:
   `#[component]` turns `message` into a unit struct, and the example imports
   it. So Rust read the pattern as "match that value", not "bind a new
   variable". Renaming it to `line` fixed it.
   Lowercase names in patterns bind new variables only when nothing in scope
   already uses that name.
