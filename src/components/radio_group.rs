// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// A neobrutalism group of options that allows one selection.
///
/// Native radios form a group by sharing a `name`, so give every
/// [`radio_group_item`] the same `name` attribute, and mark the initially
/// selected one with `checked=""` (Octane's `defaultValue`). Arrow keys move
/// between items of the same name natively. `attrs` are forwarded to the
/// `<div role="radiogroup">`, with extra classes appended to its classes;
/// name the group with `aria-label` or `aria-labelledby`.
///
/// ```ignore
/// view! {
///     radio_group(
///         <div class="flex items-center space-x-2">
///             radio_group_item(attrs: attributes! { id="r1" name="density" value="default" })
///             label(attrs: attributes! { for="r1" }, "Default")
///         </div>
///         <div class="flex items-center space-x-2">
///             radio_group_item(
///                 attrs: attributes! { id="r2" name="density" value="comfortable" checked="" }
///             )
///             label(attrs: attributes! { for="r2" }, "Comfortable")
///         </div>
///     )
/// }
/// ```
#[component]
pub async fn radio_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="radiogroup"
            data-slot="radio-group"
            class=(class!("grid gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for the visible circle.
///
/// Base UI's `data-disabled:` becomes `has-disabled:`, and `focus-visible:`
/// and `aria-invalid:` become their `has-` forms, read from the native radio
/// inside.
const ITEM: StaticClass = class!(
    "relative inline-flex aspect-square size-4 items-center justify-center rounded-full \
     border-2 border-border text-black has-focus-visible:outline-none \
     has-focus-visible:ring-2 has-focus-visible:ring-ring has-focus-visible:ring-offset-2 \
     has-disabled:cursor-not-allowed has-disabled:opacity-50 \
     has-aria-invalid:border-red-500 has-aria-invalid:text-red-500",
);

/// Classes for the native radio: invisible, covering the circle so it takes
/// the clicks, the focus and the form value.
const INPUT: StaticClass = class!(
    "peer absolute inset-0 m-0 size-full cursor-[inherit] appearance-none rounded-full opacity-0",
);

/// Classes for the dot, shown only while the radio is selected, like Base
/// UI's unmounted indicator.
const INDICATOR: StaticClass =
    class!("pointer-events-none hidden items-center justify-center peer-checked:flex");

/// One option of a [`radio_group`], built on a native `<input type="radio">`.
///
/// Every attribute and handler in `attrs` goes on the `<input>`: `id`,
/// `name`, `value`, `checked=""`, `disabled`, `aria-invalid="true"` and
/// `@change`. Classes in `attrs` go on the visible circle instead. Pair it
/// with a `label` whose `for` matches the `id`.
#[component]
pub async fn radio_group_item(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <span data-slot="radio-group-item" class=(class!(ITEM, attrs.remove("class")))>
            <input type="radio" class=(INPUT) (attrs)>
            <span data-slot="radio-group-indicator" class=(INDICATOR)>
                // Lucide draws the circle as an unfilled outline on its own
                // element, so `[&_*]:fill-current` reaches in to fill it.
                icon(
                    data: iconify_icon!("lucide:circle"),
                    attrs: attributes! { class="size-2 fill-current text-current [&_*]:fill-current" }
                )
            </span>
        </span>
    })
}
