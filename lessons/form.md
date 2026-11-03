# Form

1. **`HashMap` for field errors.** `HashMap<&'static str, &'static str>` is
   a `Record<string, string>` with typed keys and values; the examples name
   it `Errors` with a type alias. `insert(key, value)` adds an entry, and
   `get(key)` returns an `Option<&V>`, since the key may be missing.
   `.copied()` turns that `Option<&&str>` into an `Option<&str>`, which is
   what `FormField::new` takes.

2. **`#[query_params]` derives `Deserialize`.** The struct describes the
   query string, and serde fills it in: `bool` fields parse `"true"`, unknown
   keys are ignored, and a missing key is an error unless the struct has
   `#[serde(default)]`, which takes it from the struct's `Default` instead
   (derived, or written by hand with `impl Default` as in the complex
   example). In an app, `Form<T>` reads a posted body the same way; it needs
   `serde = { version = "1", features = ["derive"] }` in `Cargo.toml` and
   `#[derive(serde::Deserialize)]` on the struct.

3. **Chaining `Option` and `Result` methods.** Where TypeScript reaches for
   `?.` and `??`, Rust chains methods: `.ok()` turns a `Result` into an
   `Option`, `.filter(|_| cond)` drops the value unless `cond` holds,
   `.map(validate)` passes a function by name, `.map_or("", |v| ..)` and
   `.unwrap_or_default()` supply the fallback, and `.is_ok_and(|v| ..)`
   tests a `Result` in one call.

4. **Characters are not bytes.** `str::len()` counts UTF-8 bytes, so the
   rules use `.chars().count()` to count characters ("é" is 2 bytes, 1
   char). JavaScript's `.length` counts UTF-16 units, a third answer.

5. **`{:#?}` prints any `Debug` value.** `#[derive(Debug)]` lets
   `format!("{value:#?}")` pretty-print a struct with its field names: the
   examples' stand-in for `JSON.stringify(data, null, 2)`.

6. **`#[serde(flatten)]` and `map_while`.** A struct has one field per key,
   so the array example's `email-0`, `email-1`, ... are gathered into a
   `HashMap` field marked `flatten`. `(0..)` is an endless range, and
   `.map_while(|i| fields.get(..).cloned())` reads indexes until the first
   missing one, then stops.
