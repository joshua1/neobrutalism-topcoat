# neobrutalism (Topcoat UI registry)

Neobrutalism components for [Topcoat](https://github.com/tokio-rs/topcoat),
ported from the Octane components in
[neobrutalism-octane](https://github.com/joshua1/neobrutalism-octane)
(`src/components/ui`). All 63 components are here, from Button to Sidebar.

**Docs:** https://neobrutalism-topcoat.vercel.app

This crate is a topcoat-ui **registry**: `topcoat ui add` copies component
sources into your app, the same way Topcoat's built-in components are
installed. Once installed, a component is your code to edit.

```
neobrutalism-topcoat/
├── registry.toml            the components and theme `topcoat ui` can install
├── themes/neobrutalism.css  the theme (tokens + Tailwind setup)
├── src/components/*.rs      the component sources
├── site/                    the docs site (a Topcoat app that installs them)
├── lessons/                 a short Rust lesson per component; start at lessons/README.md
├── CONVENTIONS.md           how components are written and the gotchas found on the way
└── scripts/                 sandbox.sh, sync.sh, gen_catalog.py
```

## Use it in a Topcoat app (spendeed, kred, ...)

1. Depend on this crate **directly** (the CLI only sees direct dependencies)
   and enable Topcoat's `ui`, `tailwind` and `icon-iconify` features:

   ```toml
   [dependencies]
   topcoat = { version = "0.10.0", features = ["ui", "tailwind", "icon-iconify"] }
   neobrutalism = { git = "https://github.com/joshua1/neobrutalism-topcoat" }
   # or, from a local checkout: neobrutalism = { path = "../neobrutalism-topcoat" }

   [build-dependencies]
   topcoat = { version = "0.10.0", default-features = false, features = ["tailwind", "icon-iconify"] }
   ```

2. Build Tailwind and stage the Lucide icons in `build.rs`:

   ```rust
   fn main() {
       for path in ["src", "styles.css", "build.rs"] {
           println!("cargo:rerun-if-changed={path}");
       }
       topcoat::tailwind::BuildConfig::new().input("styles.css").render().unwrap();
       topcoat::icon::iconify::BuildConfig::new().icon_set("lucide").stage().unwrap();
   }
   ```

3. Install the theme. `topcoat ui init` (Topcoat 0.10) only installs themes
   from the built-in registry, so run it once and then copy ours over the
   `styles.css` it wrote. To use your own colors, radius, shadow and font,
   make a theme on the docs site's **Styling** page (`/styling`) and copy or
   download its `styles.css` instead:

   ```sh
   topcoat ui init
   curl -fsSL https://raw.githubusercontent.com/joshua1/neobrutalism-topcoat/main/themes/neobrutalism.css -o styles.css
   ```

   An app that already has its own theme (spendeed does) can merge the
   `:root` tokens and `@theme inline` block into it instead.

4. In the root layout, load the stylesheet, the DM Sans font, and
   `topcoat::runtime::script()`. Add `.assets(AssetBundle::load().unwrap())`
   and `.runtime()` to the router. The docs site's `site/src/app.rs` is a
   complete example.

5. Add components and use them:

   ```sh
   topcoat ui add button card dialog --registry neobrutalism
   topcoat ui add --all --registry neobrutalism
   ```

   ```rust
   use crate::components::button::{ButtonSize, ButtonVariant, button};

   view! {
       button(variant: ButtonVariant::Reverse, size: ButtonSize::Lg, "Save")
   }
   ```

   `topcoat ui list --installed` shows which installed components have
   changed in the registry; `topcoat ui add <name> --overwrite --registry
   neobrutalism` updates one (and discards your local edits to it).

### Things to know

- **No tailwind-merge.** `class!` concatenates. To override a class a
  component already sets, use Tailwind's important suffix: `size-6!`.
- **No Base UI.** Behavior comes from the platform: `<dialog>`, `<details>`,
  the popover API, CSS anchor positioning, customizable `<select>`, native
  form controls, plus small inline scripts. Each component's doc comment
  lists what differs from the Octane version and what needs a modern
  browser.
- **Inline scripts.** Several components use inline event handlers, so a
  Content-Security-Policy has to allow them.
- **Apps that already use Topcoat's built-in components** (spendeed has them
  in `src/components/ui/`) can keep both sets side by side, because
  `components.toml` tracks each component by registry and file. Point the
  install directory at a new folder, add ours, and declare the module once:

  ```sh
  # components.toml: components_dir = "src/components/ui"  ->  "src/components/neo"
  topcoat ui add button card dialog --registry neobrutalism
  # writes src/components/neo/*.rs and src/components/neo.rs
  # then add `pub mod neo;` next to `pub mod ui;` in src/components.rs
  ```

  Use them as `crate::components::neo::button::button`. Set
  `components_dir` back to `src/components/ui` before adding more built-in
  components.

## Docs site

`site/` documents every component with live previews, their source, and the
install command, generated from the Octane docs (`src/markdown/components` in
a [neobrutalism-octane](https://github.com/joshua1/neobrutalism-octane)
checkout next to this one, or wherever `NEOBRUTALISM_OCTANE` points).
Its `/styling` page is the theme generator: pick any colors (or start from
one of 30 palettes), radius, shadow, font weights and font, see the whole site
restyle live, and copy the resulting `styles.css`. The generator lives in
`site/src/theme.rs`.

```sh
cd site
topcoat dev            # http://127.0.0.1:3000
```

After changing a component, an example or the theme:

```sh
scripts/sync.sh        # reinstall every component into site/ and regenerate the catalog
```

## Developing

- Each component lives in `src/components/<name>.rs`, is listed in
  `registry.toml` (with its `dependencies`), and is declared in
  `src/components.rs`.
- `cargo check-components` (an alias for `cargo test --features
  stage-icons`) compiles every component, so a broken one fails here before
  anyone installs it.
- `scripts/sandbox.sh <dir> <component>...` compiles a few components and
  their docs examples in a scratch app.
- Examples live in `site/src/examples/<name>.rs`: one `#[component]` per
  Octane example file.

## Deploying the docs site

The `Dockerfile` builds `site/` in release mode with its asset bundle. The
site is deployed on Vercel's Container runtime from this directory:

```sh
vercel deploy --prod
```
