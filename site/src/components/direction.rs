// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    view::{
        AttributeValueViewParts, Attributes, Child, PartsWriter, View, class, component, view,
    },
};

/// The text direction of a [`direction_provider`]'s content.
///
/// [`Default`] is `Direction::Ltr`, used when no direction is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Direction {
    /// Left to right, as in English.
    #[default]
    Ltr,
    /// Right to left, as in Arabic or Hebrew.
    Rtl,
}

impl Direction {
    /// The HTML `dir` value.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ltr => "ltr",
            Self::Rtl => "rtl",
        }
    }
}

/// Lets a `Direction` be used directly as an attribute value: `dir=(direction)`.
impl AttributeValueViewParts for Direction {
    fn attribute_present(&self) -> bool {
        true
    }

    fn into_view_parts(self, _cx: &Cx, parts: &mut PartsWriter<'_>) {
        parts.push_static_str(self.as_str());
    }
}

/// Sets the text direction for everything inside it.
///
/// Renders a `<div dir="..">` with `display: contents`, so it adds no box to
/// the layout. Content uses the direction through CSS: logical utilities
/// (`ms-2`, `start-0`, `text-start`) and `rtl:` / `ltr:` variants flip with
/// it. `attrs` are forwarded to the `<div>`.
///
/// There is no `use_direction`: the server already knows the direction it
/// renders, so pass the same [`Direction`] to any component that needs it.
///
/// ```ignore
/// view! {
///     direction_provider(direction: Direction::Rtl,
///         <p>"مرحبا"</p>
///     )
/// }
/// ```
#[component]
pub async fn direction_provider(
    #[default] direction: Direction,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            dir=(direction)
            data-slot="direction-provider"
            class=(class!("contents", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}
