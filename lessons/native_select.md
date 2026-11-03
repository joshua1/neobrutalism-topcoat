# Native Select

1. **`fn as_str(self) -> &'static str`.** The enum turns itself into the
   `data-size` text. Every arm returns a string literal, so the result is
   `'static` and can be returned freely. `self` is taken by value, which is
   cheap because the enum is `Copy`.

2. **Grouped imports.** `use topcoat::icon::{icon, iconify::iconify_icon};`
   imports several items from one path at once, reaching into nested modules
   inside the braces. It is like several `import { .. } from ".."` lines
   folded together.

3. **Macros that check at compile time.** `iconify_icon!("lucide:chevron-down")`
   looks the icon up while compiling and embeds its SVG. A misspelled icon
   name fails the build instead of showing nothing at runtime. A trailing
   `!` always marks a macro call.
