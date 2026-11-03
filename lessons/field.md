# Field

1. **Slices: `&[&str]`.** `errors: &[&str]` borrows a list of string slices
   without owning it. The caller writes `&["a", "b"]`, a borrowed array, and
   no `Vec` is allocated. The default is the empty slice `&[]`.

2. **Lifetimes you write yourself.**
   `fn unique_messages<'a>(errors: &[&'a str]) -> Vec<&'a str>` says the
   returned strings are borrowed from the same place as the input strings.
   The slice holding them can go away sooner. Rust can't guess which borrow
   the result comes from, so `'a` says it.

3. **De-duplicating with a `HashSet`.** `seen.insert(x)` returns `true` the
   first time it sees `x` and `false` after that, so
   `.filter(|m| seen.insert(*m))` keeps the first copy of each message in
   order. `*m` dereferences: it takes the value out of the reference
   `filter` hands you.

4. **Slice patterns.** `match messages.as_slice() { [] => .., [message] =>
   .., messages => .. }` matches on the *shape* of a list: empty, exactly
   one item (bound to `message`), or anything else.

5. **`use super::label::label`.** `super` is the parent module
   (`components`), so this reuses the sibling `label` component, like
   `import { Label } from "./label"`.
