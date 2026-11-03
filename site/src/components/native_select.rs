// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// The height of a [`native_select`].
///
/// [`Default`] is `NativeSelectSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NativeSelectSize {
    /// The standard height, matching `input`.
    #[default]
    Default,
    /// A compact height.
    Sm,
}

impl NativeSelectSize {
    /// The `data-size` value. The select's classes style each size through it.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

/// Classes for the `<select>`: the thick border, room for the chevron, and
/// the compact `data-[size=sm]` height.
const SELECT: StaticClass = class!(
    "h-10 w-full min-w-0 appearance-none rounded-base border-2 border-border \
     bg-secondary-background py-2 pr-9 pl-3 text-sm font-base text-foreground \
     transition-colors outline-none select-none selection:bg-main \
     selection:text-main-foreground placeholder:text-foreground/50 \
     focus-visible:outline-hidden focus-visible:ring-2 focus-visible:ring-black \
     focus-visible:ring-offset-2 disabled:pointer-events-none disabled:cursor-not-allowed \
     data-[size=sm]:h-9 data-[size=sm]:py-1",
);

/// Classes for the wrapper that positions the chevron and dims a disabled select.
const WRAPPER: StaticClass =
    class!("group/native-select relative w-fit has-[select:disabled]:opacity-50");

/// Classes for the option and group fills, which follow the system colors
/// because browsers draw the open list themselves.
const OPTION: StaticClass = class!("bg-[Canvas] text-[CanvasText]");

/// A neobrutalism dropdown built on a real `<select>`.
///
/// Pass [`native_select_option`] and [`native_select_opt_group`] as children.
/// Classes in `attrs` go on the wrapper `<div>` (to size it, e.g.
/// `w-full max-w-xs`); every other attribute and handler goes on the
/// `<select>`. Its `@change` event fires when the user picks an option.
///
/// ```ignore
/// view! {
///     native_select(attrs: attributes! { name="fruit" class="w-full max-w-xs" },
///         native_select_option(attrs: attributes! { value="" }, "Select a fruit")
///         native_select_option(attrs: attributes! { value="apple" }, "Apple")
///     )
/// }
/// ```
#[component]
pub async fn native_select(
    #[default] size: NativeSelectSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            class=(class!(WRAPPER, attrs.remove("class")))
            data-slot="native-select-wrapper"
            data-size=(size.as_str())
        >
            <select
                data-slot="native-select"
                data-size=(size.as_str())
                class=(SELECT)
                (attrs)
            >
                (child)
            </select>
            icon(
                data: iconify_icon!("lucide:chevron-down"),
                attrs: attributes! {
                    class="pointer-events-none absolute top-1/2 right-3 size-4 \
                        -translate-y-1/2 text-foreground select-none"
                    data-slot="native-select-icon"
                }
            )
        </div>
    })
}

/// One `<option>` of a [`native_select`].
///
/// Pass `value` (and `disabled` or `selected`) in `attrs` and the label as
/// children.
#[component]
pub async fn native_select_option(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <option
            data-slot="native-select-option"
            class=(class!(OPTION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </option>
    })
}

/// A labelled `<optgroup>` of [`native_select_option`]s.
///
/// Pass the group's `label` in `attrs` and its options as children.
#[component]
pub async fn native_select_opt_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <optgroup
            data-slot="native-select-optgroup"
            class=(class!(OPTION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </optgroup>
    })
}
