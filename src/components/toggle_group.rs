// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

use super::toggle::{ToggleKind, ToggleSize, ToggleVariant, toggle_variants};

/// The direction a [`toggle_group`] lays its items out in.
///
/// [`Default`] is `ToggleGroupOrientation::Horizontal`, used when no
/// orientation is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToggleGroupOrientation {
    /// Items sit in a row.
    #[default]
    Horizontal,
    /// Items stack in a column.
    Vertical,
}

/// A set of related toggles, laid out together.
///
/// Rendered as a `<fieldset>`, so `disabled=""` in `attrs` disables every item.
/// `spacing` is the gap between items in Tailwind spacing units (default `2`);
/// at `0` the items join into one segmented control. Selection lives in the
/// items: give them `kind: ToggleKind::Exclusive` and a shared `name` for a
/// single choice, or leave them independent for several. Octane's group-level
/// `variant` and `size` are set on each item instead.
///
/// `attrs` are forwarded to the `<fieldset>`, with extra classes appended to
/// its classes.
///
/// ```ignore
/// view! {
///     toggle_group(spacing: 0,
///         for (value, text) in [("left", "Left"), ("right", "Right")] {
///             toggle_group_item(
///                 kind: ToggleKind::Exclusive,
///                 attrs: attributes! { name="align" value=(value) aria-label=(text) },
///                 (text)
///             )
///         }
///     )
/// }
/// ```
#[component]
pub async fn toggle_group(
    /// The gap between items, in Tailwind spacing units.
    #[default(2)]
    spacing: u8,
    /// Whether the items sit in a row or a column.
    #[default]
    orientation: ToggleGroupOrientation,
    /// Extra attributes for the `<fieldset>`.
    #[default]
    mut attrs: Attributes,
    /// The [`toggle_group_item`]s.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset
            data-slot="toggle-group"
            data-spacing=(spacing)
            match orientation {
                ToggleGroupOrientation::Horizontal => data-orientation="horizontal",
                ToggleGroupOrientation::Vertical => data-orientation="vertical",
            }
            style=(format!("--gap: {spacing}"))
            class=(class!(
                "group/toggle-group flex w-fit flex-row items-center gap-[--spacing(var(--gap))] \
                 rounded-base data-[orientation=vertical]:flex-col \
                 data-[orientation=vertical]:items-stretch m-0 min-w-0 border-0 p-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </fieldset>
    })
}

/// Classes that join items into a segmented control when the group's spacing
/// is 0, and raise the focused item above its neighbours.
const ITEM: StaticClass = class!(
    "shrink-0 group-data-[spacing=0]/toggle-group:rounded-none \
     group-data-[spacing=0]/toggle-group:px-2 has-focus-visible:z-10 \
     group-data-[spacing=0]/toggle-group:has-data-[icon=inline-end]:pr-1.5 \
     group-data-[spacing=0]/toggle-group:has-data-[icon=inline-start]:pl-1.5 \
     group-data-[orientation=horizontal]/toggle-group:group-data-[spacing=0]/toggle-group:first:rounded-l-base \
     group-data-[orientation=vertical]/toggle-group:group-data-[spacing=0]/toggle-group:first:rounded-t-base \
     group-data-[orientation=horizontal]/toggle-group:group-data-[spacing=0]/toggle-group:last:rounded-r-base \
     group-data-[orientation=vertical]/toggle-group:group-data-[spacing=0]/toggle-group:last:rounded-b-base \
     group-data-[orientation=horizontal]/toggle-group:group-data-[spacing=0]/toggle-group:border-l-0 \
     group-data-[orientation=vertical]/toggle-group:group-data-[spacing=0]/toggle-group:border-t-0 \
     group-data-[orientation=horizontal]/toggle-group:group-data-[spacing=0]/toggle-group:first:border-l-2 \
     group-data-[orientation=vertical]/toggle-group:group-data-[spacing=0]/toggle-group:first:border-t-2",
);

/// One toggle of a [`toggle_group`].
///
/// A `<label>` around a visually hidden checkbox, or a radio button for
/// `kind: ToggleKind::Exclusive`. Classes in `attrs` apply to the `<label>`;
/// every other attribute (`name`, `value`, `checked=""`, `aria-label`,
/// `disabled=""`) goes on the `<input>`.
#[component]
pub async fn toggle_group_item(
    /// Whether the item presses on its own or as one of the group's choices.
    #[default]
    kind: ToggleKind,
    /// The visual style.
    #[default]
    variant: ToggleVariant,
    /// The dimensions.
    #[default]
    size: ToggleSize,
    /// Extra attributes for the `<input>`; classes go on the `<label>`.
    #[default]
    mut attrs: Attributes,
    /// The item's content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="toggle-group-item"
            class=(class!(ITEM, toggle_variants(variant, size), attrs.remove("class")))
        >
            <input type=(kind.input_type()) class="sr-only" (attrs)>
            (child)
        </label>
    })
}
