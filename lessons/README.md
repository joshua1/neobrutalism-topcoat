# Rust lessons

One short lesson per component, written while porting it. Each covers only the Rust
ideas that were new at that point, so reading them in this order builds up from
enums and `match` to generics, iterators, closures and shared runtime state.

1. [Button](button.md): Enums replace `cva` variants; `#[derive(...)]` generates code for you; `impl` + `match` maps each variant to classes; `const BASE: StaticClass = class!(...)`; `#[component] pub async fn button(...)`; `mut attrs` and `attrs.remove("class")`; `Result<impl View>` and `Ok(...)`; `Child<'_>` is a first look at lifetimes; `#[cfg(test)]` and `[dev-dependencies]`; `bool::then_some` turns a condition into an `Option`
2. [Badge](badge.md): `///` doc comments and `[`badge`]` links; Generic types holding a tuple: `Class<(StaticClass, StaticClass)>`; `#[must_use]`; `#[allow(dead_code)]`; No `render` prop
3. [Card](card.md): `&'static str`: text that lives forever; `const fn`; Private methods on a public type; `use` paths and grouped imports
4. [Alert](alert.md): `\` at the end of a line inside a string; `Self` inside an `impl`; A variant named `Default` vs the `Default` trait
5. [Kbd](kbd.md): A `!` means a macro; Checks at compile time; Strings are UTF-8
6. [Label](label.md): `let` bindings are immutable by default; Use a value before you give it away; `if` works on attributes, too
7. [Input](input.md): A component without `child` takes no children; Long strings across lines with `\`; Keywords inside macros
8. [Textarea](textarea.md): `#[default]` uses the `Default` trait; String literals are `&'static str`; Enumerated attributes take strings
9. [Skeleton](skeleton.md): Named arguments come from the macro; `attributes! { .. }` builds a value; Text vs code in markup
10. [Spinner](spinner.md): `#[default(expr)]` for a custom default; Making a `String` from a literal; `into_*` methods consume their value
11. [Avatar](avatar.md): Required props; `String` vs `&str`, and `#[into]`; Arrays of tuples and destructuring; Name clashes and `self as ui`
12. [Breadcrumb](breadcrumb.md): `Option<T>` replaces `children ?? <ChevronRight />`; `match` on an `Option`; `From<T> for Option<T>`; Helper functions instead of a `render` prop
13. [Empty](empty.md): Item order doesn't matter; No HTML escaping by hand; Text and elements together
14. [Table](table.md): `struct` with named fields; Fixed-size arrays; Borrowing in a loop, and iterators; `format!` builds a `String`
15. [Progress](progress.md): `Expr<f64>`: one prop, static or live; `f64` and no implicit number conversion; Moves and `.clone()`; Typed closures; `parse()` returns a `Result`
16. [Checkbox](checkbox.md): `Option` instead of `undefined`; One `attrs`, two destinations; Named arguments to nested components
17. [Switch](switch.md): Several methods on one enum; Type inference in expressions; `switch` is a plain name
18. [Radio Group](radio_group.md): Arrays of tuples and destructuring; `==` compares contents; A pattern name can mean an existing item; `#[key(value)]` on a loop
19. [Native Select](native_select.md): `fn as_str(self) -> &'static str`; Grouped imports; Macros that check at compile time
20. [Field](field.md): Slices: `&[&str]`; Lifetimes you write yourself; De-duplicating with a `HashSet`; Slice patterns; `use super::label::label`
21. [Input Group](input_group.md): `Option<T>` as an optional prop; Nested `use` paths; A `const` always has a type
22. [Toggle](toggle.md): A `pub` method another file can call; Tuple structs (newtypes); Arms may return the same value
23. [Toggle Group](toggle_group.md): `use super::toggle::{...}`; Sized integers and `#[default(2)]`; `format!` builds a `String`; Signals and closures in the `custom` example
24. [Button Group](button_group.md): One enum, several methods; A prop default other than the type's `Default`; Enum paths as values
25. [Accordion](accordion.md): `bool` props become boolean attributes; `Option` for attributes that need a value; Fixed-size arrays of tuples; Destructuring in `for`
26. [Collapsible](collapsible.md): `mut` only where you change something; Attribute lists are maps; Plain `&str` constants work as classes
27. [Tabs](tabs.md): `Expr<bool>`: a value now or a value later; `if` at the node level picks an element; `&self` vs `&mut self` methods; `let` with an inferred, unnamed type; `match` in an attribute list
28. [Dialog](dialog.md): Custom defaults: `#[default(expr)]`; `#[into]` and the `From` / `Into` traits; Borrow, then move; `&'static str`; `pub const` and `\` line continuations
29. [Alert dialog](alert_dialog.md): `use super::...` imports a sibling module; Components wrap components; Later entries win in `attributes!`; A default per component; Enums to strings with `as_str`
30. [Sheet](sheet.md): `pub` decides what other modules can see; Enums in data; `format!` builds a `String`; `String` vs `&str` at call sites; Ranges and `_`
31. [Drawer](drawer.md): `Option<bool>` for "unset" vs `false`; `matches!`; `if` is an expression; Moving in exclusive branches; Naming a generic return type
32. [Popover](popover.md): Returning an owned `String`; `then` vs `then_some`; Matching a tuple; Importing variants locally
33. [Tooltip](tooltip.md): Or-patterns; A component that adds nothing; Types are nominal
34. [Hover card](hover_card.md): Integer types; Literals take their type from context; `&str` is `Copy`
35. [Dropdown Menu](dropdown_menu.md): Matching on a tuple: `match (side, align)`; `use` inside a function; `#[default(true)]` for a non-zero default; `bool::then_some` turns a flag into an `Option`; Long string literals with `\`
36. [Context Menu](context_menu.md): `&'static str` return types; The same method name on different types; Arrays of tuples and destructuring in `for`; Owned `String` vs borrowed `&str`
37. [Menubar](menubar.md): Items are private to their module by default; Duplication over coupling, on purpose; Constants always spell out their type
38. [Navigation Menu](navigation_menu.md): Components can call components; A function that returns a constant; Private helper components; Array constants with a fixed length
39. [Select](select.md): `cx: &Cx`, the request context; Building a `String` piece by piece; Raw strings: `r#"..."#`; `Option<&str>` with `#[into]` and `if let`
40. [Pagination](pagination.md): Conditional entries in `class!`; Ranges: `1..=5` and `0..100`; Numbers render directly
41. [Slider](slider.md): `Vec<f64>` and float literals; Custom defaults: `#[default(100.0)]`; Closures; Iterator chains are lazy; Shadowing; `mut` inside a pattern
42. [Input OTP](input_otp.md): Escapes vs raw strings; `char` is not a one-letter `String`; `collect` builds a `String`, too; `bool` to `Option`; Ranges
43. [Scroll area](scroll_area.md): `struct`: a named record; Fixed-size arrays: `[Artwork; 3]`; `format!` is a template literal; A component with no children
44. [Item](item.md): `&'static str`; Private and public methods in one `impl`; Class lists nest; A `render` prop becomes a recipe
45. [Image card](image_card.md): Required props; `&str` is a borrowed string; Interpolation escapes; `///` on parameters
46. [Marker](marker.md): `Option<T>` instead of `undefined`; `class!` skips `None`; Types record the shape; The `!` means macro
47. [Bubble](bubble.md): `#[default(expr)]`; Not every enum derives `Default`; One enum, several methods; `pub const` as a public class list
48. [Message](message.md): `crate::` and `super::` paths; Private helper components; Arrays of tuples and destructuring; `#[key(label)]` on loops that call components
49. [Attachment](attachment.md): Building on another component; Forwarding through a wrapped component; `let` before `view!`; `#[default]` on any variant; No tailwind-merge
50. [Marquee](marquee.md): Slices: `&[&str]`; Borrowing lets you loop twice; `*item` dereferences
51. [Direction](direction.md): Implementing a library trait for your type; `&self` vs `self`; `_cx`: deliberately unused; No `useDirection` hook
52. [Message scroller](message_scroller.md): Raw strings: `r#"..."#`; `match` in an attribute list; Shards and signals; Iterator chains; Patterns can collide with names
53. [Command](command.md): `Unescaped::new_unchecked`: opting out of escaping; Type aliases; `Clone` but not `Copy`; Arrays coerce to slices
54. [Combobox](combobox.md): `unwrap_or_default()`; Slicing a string by bytes; Nested fixed-size arrays; One script, many items
55. [Toast](toast.md): A method is a function value; `move` closures; Patterns in closure parameters; `Option` of a tuple; `Option` attributes with numbers
56. [Calendar](calendar.md): `div_euclid` and `rem_euclid` for negative numbers; `#[derive(PartialOrd, Ord)]` on a struct; `Display` and `FromStr` give you `to_string()` and `parse()`; `?` with your own error: `ok_or` and `map_err`; `as_deref`: from `Option<String>` to `Option<&str>`
57. [Date picker](date_picker.md): A fresh identity with `cx.keyed(..)`; Splitting on several characters; `find` returns an `Option`
58. [Carousel](carousel.md): `Option::map` turns an optional prop into an optional attribute; Names inside `format!` braces; JavaScript `${..}` inside a Rust string is just text
59. [Resizable](resizable.md): Floats are not `Ord`; A method that returns another variant; Floats in `format!`
60. [Chart](chart.md): `static` vs `const`; Associated constants; `write!` into a `String`; `windows`, `zip` and `fold`; Floating-point noise
61. [Form](form.md): `HashMap` for field errors; `#[query_params]` derives `Deserialize`; Chaining `Option` and `Result` methods; Characters are not bytes; `{:#?}` prints any `Debug` value; `#[serde(flatten)]` and `map_while`
62. [Data Table](data_table.md): A generic struct: `Column<T>`; Function pointers: `fn(&T) -> String`; Sorting with `Ordering`; Iterators for the row model; Struct update syntax; `let ... else` and `let` chains
63. [Questionnaire](questionnaire.md): `expr!` outside `view!`; Blocks are expressions; `else if` chains return values; Rust strings and browser strings agree
64. [Sidebar](sidebar.md): `macro_rules!` and `concat!`: strings built at compile time; `?` works on `Option` too; Constants as patterns; A plain function can build a view; Randomness without a crate
65. [Theme generator](theme_generator.md): The newtype pattern: `struct Hex(String)`; `impl Display` makes a type printable; Function pointers in data: `get: fn(&Theme) -> &Hex`; Pattern tricks for parsing; A generic function with trait bounds; `include_str!` embeds a file at compile time; Unit tests next to the code
