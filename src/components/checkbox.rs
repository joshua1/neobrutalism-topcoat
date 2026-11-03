// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, StaticClass, View, attributes, class, component, view},
};

/// Classes for the visible box, the `peer` that a following label reads.
///
/// Base UI's `data-checked:`, `data-disabled:` and `data-indeterminate:` become
/// `has-checked:`, `has-disabled:` and `has-indeterminate:`, read from the
/// native input inside, and `focus-visible:` / `aria-invalid:` become their
/// `has-` forms for the same reason.
const ROOT: StaticClass = class!(
    "peer relative flex size-4 shrink-0 items-center justify-center outline-2 outline-border \
     ring-offset-white has-focus-visible:outline-hidden has-focus-visible:ring-2 \
     has-focus-visible:ring-black has-focus-visible:ring-offset-2 \
     has-disabled:cursor-not-allowed has-disabled:opacity-50 \
     has-checked:bg-main has-checked:text-white \
     has-indeterminate:bg-main has-indeterminate:text-white \
     has-aria-invalid:outline-red-500",
);

/// Classes for the native checkbox: invisible, covering the box so it takes
/// the clicks, the focus and the form value.
const INPUT: StaticClass = class!(
    "peer absolute inset-0 m-0 size-full cursor-[inherit] appearance-none opacity-0",
);

/// Classes for the check mark, shown only while the input is checked (or
/// indeterminate), like Base UI's unmounted indicator.
const INDICATOR: StaticClass = class!(
    "pointer-events-none hidden items-center justify-center text-current \
     peer-checked:flex peer-indeterminate:flex",
);

/// A neobrutalism checkbox built on a native `<input type="checkbox">`.
///
/// Every attribute and handler in `attrs` goes on the `<input>`: `id`,
/// `name`, `value`, `checked=""` for the initial state (Octane's
/// `defaultChecked`), `disabled`, `aria-invalid="true"`, and `@change` to
/// react to changes (`e.target.checked`). Bind `:checked=$(..)` to control it
/// from a signal. Classes in `attrs` go on the visible box instead.
///
/// The indeterminate state is the DOM `indeterminate` property, which HTML
/// cannot set: bind it with `:indeterminate=$(..)` (the runtime sets it as a
/// property). It shares the checked style and check mark.
///
/// ```ignore
/// view! {
///     <div class="flex items-center gap-2">
///         checkbox(attrs: attributes! { id="terms" name="terms" checked="" })
///         label(attrs: attributes! { for="terms" }, "Accept terms and conditions")
///     </div>
/// }
/// ```
#[component]
pub async fn checkbox(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <span data-slot="checkbox" class=(class!(ROOT, attrs.remove("class")))>
            <input type="checkbox" class=(INPUT) (attrs)>
            <span data-slot="checkbox-indicator" class=(INDICATOR)>
                icon(
                    data: iconify_icon!("lucide:check"),
                    attrs: attributes! { class="size-4 text-main-foreground" }
                )
            </span>
        </span>
    })
}
