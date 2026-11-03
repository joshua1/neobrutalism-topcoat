// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use std::collections::HashSet;

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::label::label;

/// A semantic group of related fields, named by a [`field_legend`].
///
/// `attrs` are forwarded to the `<fieldset>`, with extra classes appended to
/// its classes. Pass `disabled` to disable all of its controls at once.
///
/// ```ignore
/// view! {
///     field_set(
///         field_legend("Contact us")
///         field_description("Fill in the form and we will get back to you.")
///         field_group(
///             field(
///                 field_label(attrs: attributes! { for="name" }, "Name")
///                 input(attrs: attributes! { id="name" placeholder="John Doe" })
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn field_set(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset
            data-slot="field-set"
            class=(class!(
                "flex flex-col gap-4 has-[>[data-slot=checkbox-group]]:gap-3 \
                 has-[>[data-slot=radio-group]]:gap-3",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </fieldset>
    })
}

/// The text size of a [`field_legend`].
///
/// [`Default`] is `FieldLegendVariant::Legend`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FieldLegendVariant {
    /// A heading for a section of the form.
    #[default]
    Legend,
    /// A smaller heading, the size of a field label.
    Label,
}

impl FieldLegendVariant {
    /// The `data-variant` value. The legend's classes style each variant
    /// through it, and [`field_description`] tucks up under a `legend` one.
    fn as_str(self) -> &'static str {
        match self {
            Self::Legend => "legend",
            Self::Label => "label",
        }
    }
}

/// The heading of a [`field_set`]. Place it first in the set.
///
/// `variant` defaults to `Legend`; `Label` matches the size of a field label.
#[component]
pub async fn field_legend(
    #[default] variant: FieldLegendVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <legend
            data-slot="field-legend"
            data-variant=(variant.as_str())
            class=(class!(
                "mb-1.5 font-heading data-[variant=label]:text-sm data-[variant=legend]:text-base",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </legend>
    })
}

/// A stack of [`field`]s. It is also the container that
/// `FieldOrientation::Responsive` fields measure.
#[component]
pub async fn field_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="field-group"
            class=(class!(
                "group/field-group @container/field-group flex w-full flex-col gap-5 \
                 data-[slot=checkbox-group]:gap-3 *:data-[slot=field-group]:gap-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The layout of a [`field`].
///
/// [`Default`] is `FieldOrientation::Vertical`, used when no orientation is
/// given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum FieldOrientation {
    /// Stack the label, control and supporting text.
    #[default]
    Vertical,
    /// Place the control beside its label or [`field_content`].
    Horizontal,
    /// Stack, then switch to a row once the enclosing [`field_group`] is at
    /// least `md` wide.
    Responsive,
}

impl FieldOrientation {
    /// The `data-orientation` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Vertical => "vertical",
            Self::Horizontal => "horizontal",
            Self::Responsive => "responsive",
        }
    }

    /// Classes for the orientation's layout.
    ///
    /// Base UI controls carry `role="checkbox"` / `role="radio"`; the native
    /// controls here are wrapped, so the nudge targets their `data-slot`
    /// wrappers instead.
    fn classes(self) -> StaticClass {
        match self {
            Self::Vertical => class!("flex-col *:w-full [&>.sr-only]:w-auto"),
            Self::Horizontal => class!(
                "flex-row items-center has-[>[data-slot=field-content]]:items-start \
                 *:data-[slot=field-label]:flex-auto \
                 has-[>[data-slot=field-content]]:[&>[data-slot=checkbox],[data-slot=radio-group-item]]:mt-px",
            ),
            Self::Responsive => class!(
                "flex-col *:w-full @md/field-group:flex-row @md/field-group:items-center \
                 @md/field-group:*:w-auto \
                 @md/field-group:has-[>[data-slot=field-content]]:items-start \
                 @md/field-group:*:data-[slot=field-label]:flex-auto [&>.sr-only]:w-auto \
                 @md/field-group:has-[>[data-slot=field-content]]:[&>[data-slot=checkbox],[data-slot=radio-group-item]]:mt-px",
            ),
        }
    }
}

/// Classes shared by every field orientation.
const FIELD: StaticClass =
    class!("group/field flex w-full gap-2 font-base data-[invalid=true]:text-red-500");

/// A control with its label, description and error message.
///
/// `orientation` defaults to `Vertical`. `attrs` are forwarded to the
/// `<div role="group">`, with extra classes appended to its classes. Set
/// `data-invalid="true"` to turn the field's text red, and
/// `data-disabled="true"` to dim its label (the control itself still needs
/// `disabled`). Validation is the caller's job.
///
/// ```ignore
/// view! {
///     field(orientation: FieldOrientation::Horizontal,
///         checkbox(attrs: attributes! { id="terms" })
///         field_label(attrs: attributes! { for="terms" }, "Accept terms and conditions")
///     )
/// }
/// ```
#[component]
pub async fn field(
    #[default] orientation: FieldOrientation,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            role="group"
            data-slot="field"
            data-orientation=(orientation.as_str())
            class=(class!(FIELD, orientation.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A column holding a field's label and description, beside a control in a
/// horizontal [`field`].
#[component]
pub async fn field_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="field-content"
            class=(class!(
                "group/field-content flex flex-1 flex-col gap-0.5 leading-snug",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for a field label, including the choice-card look it takes when
/// it wraps a whole [`field`].
///
/// Base UI's `has-data-checked:` becomes `has-checked:`, read from the
/// native checkbox, radio or switch inside the card. `leading-snug!` is
/// important because it replaces the label's own `leading-none`, which
/// Octane's `cn` (tailwind-merge) would drop and `class!` does not.
const FIELD_LABEL: StaticClass = class!(
    "group/field-label peer/field-label flex w-fit gap-2 leading-snug! \
     group-data-[disabled=true]/field:opacity-50 has-[>[data-slot=field]]:rounded-base \
     has-[>[data-slot=field]]:border-2 has-[>[data-slot=field]]:border-border \
     has-[>[data-slot=field]]:bg-secondary-background \
     has-[>[data-slot=field]]:has-checked:bg-main \
     has-[>[data-slot=field]]:has-checked:text-main-foreground \
     has-[>[data-slot=field]]:has-[:focus-visible]:ring-2 \
     has-[>[data-slot=field]]:has-[:focus-visible]:ring-black \
     has-[>[data-slot=field]]:has-[:focus-visible]:ring-offset-2 \
     *:data-[slot=field]:p-2.5 has-[>[data-slot=field]]:w-full \
     has-[>[data-slot=field]]:flex-col",
);

/// A [`label`] for a field's control.
///
/// Pass `for` in `attrs` matching the control's `id`. Wrap a whole [`field`]
/// in it to make a selectable choice card, filled while its checkbox, radio
/// or switch is checked.
#[component]
pub async fn field_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        label(
            attrs: attributes! {
                data-slot="field-label"
                class=(class!(FIELD_LABEL, attrs.remove("class")))
                (attrs)
            },
            (child)
        )
    })
}

/// Label-sized text that does not label a control, such as a choice card's
/// title. Use [`field_label`] to label a control.
#[component]
pub async fn field_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="field-label"
            class=(class!(
                "flex w-fit items-center gap-2 text-sm font-heading \
                 group-data-[disabled=true]/field:opacity-50",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Supporting text for a field or field set.
///
/// Give it an `id` and reference it from the control's `aria-describedby`.
#[component]
pub async fn field_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="field-description"
            class=(class!(
                "text-left text-sm leading-normal font-base text-foreground \
                 group-has-data-horizontal/field:text-balance [[data-variant=legend]+&]:-mt-1.5",
                "last:mt-0 nth-last-2:-mt-1",
                "[&>a]:underline [&>a]:underline-offset-4 [&>a:hover]:text-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </p>
    })
}

/// A horizontal rule between sections of a form, with optional text in the
/// middle.
///
/// Pass the text as children. Without children the text box renders empty
/// and `empty:hidden` hides it.
#[component]
pub async fn field_separator(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="field-separator"
            class=(class!(
                "relative -my-2 h-5 text-sm group-data-[variant=outline]/field-group:-mb-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <div
                role="separator"
                aria-orientation="horizontal"
                class="absolute inset-x-0 top-1/2 border-t-2 border-border"
            ></div>
            <span
                class="relative mx-auto block w-fit bg-background px-2 font-base \
                    text-foreground empty:hidden"
                data-slot="field-separator-content"
            >
                (child)
            </span>
        </div>
    })
}

/// Removes empty and repeated messages, keeping the first of each in order.
fn unique_messages<'a>(errors: &[&'a str]) -> Vec<&'a str> {
    let mut seen = HashSet::new();
    errors
        .iter()
        .copied()
        .filter(|message| !message.is_empty())
        .filter(|message| seen.insert(*message))
        .collect()
}

/// An error message for a field, announced by screen readers when it
/// appears.
///
/// Pass the messages in `errors`: one unique message renders as text, more
/// render as a list, and repeats and empty strings are dropped. With no
/// `errors`, the children render instead. When neither has content the
/// alert box is empty and `empty:hidden` hides it. Give it an `id`
/// referenced by the control's `aria-describedby`.
///
/// ```ignore
/// view! {
///     field_error(errors: &["Username must be at least 3 characters."])
/// }
/// ```
#[component]
pub async fn field_error(
    #[default] errors: &[&str],
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let messages = unique_messages(errors);
    Ok(view! {
        <div
            role="alert"
            data-slot="field-error"
            class=(class!("text-sm font-base text-red-500 empty:hidden", attrs.remove("class")))
            (attrs)
        >
            match messages.as_slice() {
                [] => (child),
                [message] => (*message),
                messages => {
                    <ul class="ml-4 flex list-disc flex-col gap-1">
                        for message in messages {
                            <li>(*message)</li>
                        }
                    </ul>
                },
            }
        </div>
    })
}
