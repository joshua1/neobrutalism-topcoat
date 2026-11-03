# Label

1. **`let` bindings are immutable by default.**
   `let default_slot = !attrs.contains_key("data-slot");` computes a `bool`
   once. Without `mut`, nothing can change it afterwards, like a `const` in
   TypeScript. `!` is plain boolean "not".

2. **Use a value before you give it away.** `(attrs)` moves `attrs` into the
   `<label>`, so it can't be used after that point. The check runs first and
   saves its answer in `default_slot`. `contains_key` only *borrows* `attrs`
   to look inside, so `attrs` is still ours to move later. The compiler
   rejects code that reads a value after it has moved.

3. **`if` works on attributes, too.** Inside `view!`, an `if` block in a
   tag's attribute list emits attributes only when its condition holds. Here
   it writes `data-slot="label"` unless the caller (such as `field_label`)
   brought its own. HTML allows each attribute only once.
