# Switch

1. **Several methods on one enum.** `SwitchSize` has `as_str`,
   `root_classes` and `thumb_classes` in a single `impl` block. Each has its
   own `match`, and the compiler checks each one covers every size, so
   adding a size points you at all three.

2. **Type inference in expressions.** `class!(ROOT, size.root_classes(),
   attrs.remove("class"))` builds a value whose exact type is long and
   generic, and we never write it down. Rust infers types inside a function.
   Only signatures, `const`s and `struct` fields have to spell them out.

3. **`switch` is a plain name.** In TypeScript `switch` is reserved. Rust
   uses `match` instead, so `switch` is free to be a function name.
