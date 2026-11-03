// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// Classes for the label text, dimmed when the control before it is disabled.
///
/// `peer-disabled:` covers a bare `<input>` placed before the label.
/// `peer-has-[:disabled]:` covers the wrapped controls in this registry
/// (checkbox, switch), whose `peer` is a wrapper around the disabled input.
const LABEL: StaticClass = class!(
    "text-sm font-heading leading-none \
     peer-disabled:cursor-not-allowed peer-disabled:opacity-70 \
     peer-has-[:disabled]:cursor-not-allowed peer-has-[:disabled]:opacity-70",
);

/// A neobrutalism label for a form control.
///
/// Pass `for` in `attrs` matching the control's `id`, or wrap the control.
/// Pass the label text as children. `attrs` are forwarded to the `<label>`,
/// with extra classes appended to its classes. A `data-slot` in `attrs`
/// replaces the default `label` slot, which is how `field_label` marks
/// itself.
///
/// ```ignore
/// view! {
///     <div class="flex items-center gap-2">
///         checkbox(attrs: attributes! { id="terms" })
///         label(attrs: attributes! { for="terms" }, "Accept terms and conditions")
///     </div>
/// }
/// ```
#[component]
pub async fn label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // Each attribute may appear only once on an element, so the default slot
    // is only written when the caller did not bring its own.
    let default_slot = !attrs.contains_key("data-slot");
    Ok(view! {
        <label
            if default_slot {
                data-slot="label"
            }
            class=(class!(LABEL, attrs.remove("class")))
            (attrs)
        >
            (child)
        </label>
    })
}
