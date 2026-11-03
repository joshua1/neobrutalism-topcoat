# Image card

1. **Required props.** `image_url: &str` and `caption: &str` have no
   `#[default]`, so a call without them doesn't compile. The error names the
   missing prop. It's TypeScript's required-vs-`?` split, enforced at every
   call.

2. **`&str` is a borrowed string.** The card only reads the URL and caption
   while it renders, so it borrows them (`&str`) instead of taking ownership
   (`String`). Callers can pass a literal or `&my_string` without copying.

3. **Interpolation escapes.** `(caption)` and `src=(image_url)` are escaped for
   their position, so a caption containing `<script>` renders as text. You opt
   out only through the explicitly named `*_unescaped` APIs.

4. **`///` on parameters.** Doc comments work on a single prop too. Your
   editor shows them when you hover the argument at a call site.
