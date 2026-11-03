# Skeleton

1. **Named arguments come from the macro.** Rust functions only take
   positional arguments. `skeleton(attrs: attributes! { class="h-4" })` works
   because `#[component]` generates a builder behind the call, so you can
   name props and list them in any order, and it still checks them at
   compile time.

2. **`attributes! { .. }` builds a value.** It creates an `Attributes`
   collection right where you call the component, using the same
   `name="value"` syntax as an HTML tag. It is a value like any other: you
   could store it in a `let` first and pass it later.

3. **Text vs code in markup.** Inside `view!`, text is always quoted
   (`"Loading"`) and Rust expressions go in parentheses (`(name)`). Bare words
   are markup or component names. This is how one macro tells the three
   apart.
