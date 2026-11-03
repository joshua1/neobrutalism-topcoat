# Item

1. **`&'static str`.** `as_str` returns `"default"`, `"sm"` or `"xs"`. A string
   literal is baked into the program, so it lives for the whole run: its
   lifetime is `'static`. Returning one costs nothing, no allocation, unlike
   building a `String`.

2. **Private and public methods in one `impl`.** `classes()` has no `pub`, so
   only this file can call it. `as_str()` is `pub`, so an app can write
   `data-size=(ItemSize::Sm.as_str())` when it styles its own `<a>` as an item.
   In TypeScript everything on an object is reachable; in Rust each item opts
   in.

3. **Class lists nest.** `class!(item_variants(ItemSize::Default), "max-w-md")`
   puts a whole `Class` inside another. `class!` accepts anything that
   implements its `ClassViewParts` trait, and `Class` does, like `&str` and
   `Option`. A trait is a shared interface; types sign up to it.

4. **A `render` prop becomes a recipe.** Octane's `render={<a href="#" />}`
   has no Rust counterpart. `item_variants` gives you the classes, and the
   item's `data-slot` and `data-size` are plain attributes you add yourself.
