# Radio Group

1. **Arrays of tuples and destructuring.**
   `let options = [("default", "r1", "Default"), ..];` is a fixed-size array
   whose items are tuples. `for (value, id, text) in options` unpacks each
   tuple into three names, like `for (const [value, id, text] of options)`.

2. **`==` compares contents.** `checked=(value == "comfortable")` compares
   the text of two strings. Rust has no reference equality surprise here,
   unlike `===` on JavaScript objects. The resulting `bool` adds `checked`
   only to the matching radio.

3. **A pattern name can mean an existing item.** In a pattern, a name that
   matches a unit struct or constant in scope is a comparison against it,
   not a new variable. Each `#[component]` defines a unit struct with its own
   name, so `for (.., description) in ..` in a file that also has a
   `description` example failed to compile. Renaming the binding to `hint`
   fixed it.

4. **`#[key(value)]` on a loop.** An attribute can sit on a `for` inside
   `view!`. It gives each item a stable identity, like React's `key`.
