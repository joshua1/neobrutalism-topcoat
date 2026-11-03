# Avatar

1. **Required props.** `avatar_image(src: ..)` has no `#[default]` on `src`,
   so leaving it out is a compile error, not a broken `<img>` at runtime. The
   `#[component]` macro builds a checked argument list for every call.

2. **`String` vs `&str`, and `#[into]`.** `String` owns its text (heap
   memory it frees itself); `&str` borrows text owned elsewhere. `src` is a
   `String`, and `#[into]` lets callers pass a literal like
   `"https://github.com/shadcn.png"`, which is converted for them.

3. **Arrays of tuples and destructuring.** The group examples keep their data
   in `const PEOPLE: [(&str, &str, &str); 3]`, a fixed-size array of
   three-item tuples. `for (src, alt, initials) in PEOPLE` unpacks each tuple,
   like `for (const [src, alt, initials] of people)`.

4. **Name clashes and `self as ui`.** The examples `avatar_group` and
   `avatar_group_count` have the same names as the parts they demonstrate,
   and one scope can't hold two items with the same name.
   `use crate::components::avatar::{self as ui, ..}` imports the module
   itself as `ui`, and the examples call `ui::avatar_group(..)`.
