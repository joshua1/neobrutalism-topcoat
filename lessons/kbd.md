# Kbd

1. **A `!` means a macro.** `class!(..)`, `view! { .. }` and
   `iconify_icon!(..)` are macros: they run at compile time and write code
   for you. Ordinary functions can't take a variable number of arguments or
   HTML-like markup; macros can.

2. **Checks at compile time.** `iconify_icon!("lucide:circle-dashed")` looks
   the icon up while the program compiles. A misspelled name fails the build
   instead of rendering an empty square.

3. **Strings are UTF-8.** `kbd("⌘K")` needs no escaping: every Rust string
   is UTF-8. Note that `"⌘K".len()` is 4, because `len` counts bytes, not
   characters (`⌘` takes three bytes). Use `.chars().count()` to count
   characters.
