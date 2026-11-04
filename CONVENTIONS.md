# Porting conventions

How each Octane component in `src/components/ui/<name>.tsrx` of
[neobrutalism-octane](https://github.com/joshua1/neobrutalism-octane) becomes a
Topcoat component in `src/components/<snake_name>.rs`.

## Sources of truth

- **Look and markup:** the `.tsrx` file. Keep its class strings, variants,
  sizes, `data-slot` values and element structure. These are neobrutalism
  components, not restyled Topcoat ones.
- **Rust structure and behavior:** the matching Topcoat component, when there
  is one, in
  `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/topcoat-ui-registry-0.10.0/src/components/`.
  `src/components/button.rs` and `badge.rs` here are the house style.
- **Topcoat docs:** `topcoat-0.10.0/docs/runtime.md` (signals, `$(..)`,
  `@event`, `:bind`), `topcoat-view-macro-0.10.0/docs/{component,class,attributes}.md`.

## API shape

- File name: hyphens become underscores (`alert-dialog` -> `alert_dialog.rs`).
- Every part is `#[component] pub async fn <name>(...) -> Result<impl View>`.
  Compound parts are separate functions named after the Octane export in
  snake_case (`CardHeader` -> `card_header`).
- Props: `#[default] mut attrs: Attributes` (forwarded with `(attrs)`, caller
  classes appended via `attrs.remove("class")`), and `#[default] child: Child<'_>`.
- cva variants become `#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]`
  `#[allow(dead_code)]` enums with a private `fn classes(self) -> StaticClass`.
  Variant names are the cva keys in PascalCase (`noShadow` -> `NoShadow`,
  `icon-sm` -> `IconSm`). Export `<name>_variants(..)` when Octane exports
  `<name>Variants`.
- Shared class lists are `const NAME: StaticClass = class!(...)`.
- A component that uses another one imports it with `use super::<other>::{..}`
  and lists it as a dependency in `registry.toml`.
- Doc comments (`///`) on every public item, in the style of `button.rs`,
  with an ` ```ignore ` usage example on the main component.
- Every component file starts with a module-level `#![allow(dead_code)]`
  (after any `//!` docs), with the standard two-line comment: apps often use
  only some parts of a component, and the unused parts' helpers must not warn
  in their builds.

## Behavior without a JS component library

There is no Base UI here. Pick the first approach that works:

1. **Native HTML:** `<details>`/`<summary>` (accordion, collapsible, menus),
   `<dialog>` (dialog, sheet, drawer, alert dialog), the `popover` attribute
   with `popovertarget` buttons (popover, menus, select-like lists),
   `<input type="checkbox|radio|range">` with `peer-checked:` / `has-checked:`
   styles, `group-hover:` / `group-focus-within:` (tooltip, hover card).
2. **Topcoat runtime:** `Expr<bool>` props with `#[into]` for controlled state
   (see Topcoat's `dialog.rs`), signals and `@click=$(..)` handlers in
   examples, and `:attr=$(..)` bindings.
3. **Small inline JS** only where the first two can't do it. Use plain HTML
   event attributes, `onclick="this.closest('dialog').close()"`, where `this`
   is the element. Do **not** use `@click="..."` with a JS string for DOM
   work: the runtime evaluates that string once as an expression that must
   return a function, and the handler receives a stand-in event with no DOM
   access.

`class!` concatenates; it does not merge conflicting Tailwind classes the way
Octane's `cn` (tailwind-merge) does. When a component or example must
override a class the base already sets (e.g. `size-6` over a button's
`size-10`), use Tailwind's important suffix (`size-6!`) with a short comment.

Gotchas found while porting:

- `#[component]` defines a unit struct with the function's name. An example
  function named like a component it uses (`button`, `table`) must call that
  component by full path (`crate::components::table::table(..)`), and a loop
  binding with a component's name (`for description in ..`) becomes a pattern
  match and fails to compile; rename the binding.
- A `view!` `for` loop whose body holds signals needs `#[key(expr)]`.
- Runtime `$(..)` expressions have no `&&`; nest `if`s instead.
- Boolean HTML attributes are written `checked=""` (or `checked=(bool)`).
- A prop typed as a bare function pointer (`fn(&str) -> String`) doesn't
  compile inside `#[component]`; name it with a type alias
  (`type Format = fn(&str) -> String;`).
- `attributes!` in a plain function (not a component) needs `cx =>` first,
  and its handler closures need typed parameters (`|_e: Event|`).
- A config holding `IconData` must be a `static`, not a `const`.
- `field`'s `*:w-full` also hits a `popover_content` placed inside a field;
  give the panel `w-auto!`.
- Form controls are native inputs inside a `data-slot` wrapper (see
  `checkbox.rs`), so select them with `[data-slot=checkbox]`, not
  `[role=checkbox]`.

Translate Base UI state selectors to their native equivalents, e.g.
`data-[state=open]:` / `data-open:` -> `open:` (on `<details>`/`<dialog>`),
`group-open:` or `in-[[open]]:` for children, `data-checked:` -> `checked:`
or `has-checked:`, `data-disabled:` -> `disabled:` / `has-disabled:`.

When something can't be matched (keyboard navigation of a menu, focus
trapping, drag gestures), say so in the component's doc comment.

Icons: `use topcoat::icon::{icon, iconify::iconify_icon};` then
`icon(data: iconify_icon!("lucide:chevron-down"))`. Only Lucide is staged.

## Checking

`scripts/sandbox.sh <dir> <component>...` builds a scratch app that mounts
the listed component files (and their docs-site examples, when present).
`cargo check-components` compiles everything registered in
`src/components.rs`.

## Docs-site examples

Each component gets `site/src/examples/<snake_name>.rs`, a port of the
Octane examples in neobrutalism-octane's `src/examples/ui/<name>/`. One
`#[component] pub async fn` per Octane example file, named after the file in
snake_case (`demo.tsrx` -> `demo`). Examples import components from
`crate::components::<name>`, the way an app would after `topcoat ui add`.
Examples that need state take `cx: &Cx` and use `topcoat::runtime::signal`.

## Lessons

Each component gets `lessons/<snake_name>.md`: a brief Rust lesson (3 to 6
points) for someone coming from TypeScript, covering only the Rust ideas
that are new in that component.
