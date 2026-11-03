// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// Classes for the closed control: Octane's `SelectTrigger`, on the `<select>`
/// itself, with room on the right for the chevron.
const TRIGGER: StaticClass = class!(
    "flex h-10 w-full items-center justify-between rounded-base border-2 border-border bg-main \
     gap-2 px-3 py-2 text-sm font-base text-main-foreground ring-offset-white \
     placeholder:text-foreground/50 focus-visible:outline-hidden focus-visible:ring-2 \
     focus-visible:ring-black focus-visible:ring-offset-2 focus:outline-hidden focus:ring-2 \
     focus:ring-black focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 \
     *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex \
     *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
     appearance-none pr-9 text-left",
);

/// Classes for the open list: Octane's `SelectContent`, on the customizable
/// select's `::picker(select)`. Browsers without customizable selects ignore
/// them and show their native picker.
const PICKER: StaticClass = class!(
    "[&::picker(select)]:[appearance:base-select] [&::picker(select)]:mt-1 \
     [&::picker(select)]:max-h-96 [&::picker(select)]:overflow-x-hidden \
     [&::picker(select)]:overflow-y-auto [&::picker(select)]:rounded-base \
     [&::picker(select)]:border-2 [&::picker(select)]:border-border \
     [&::picker(select)]:bg-background [&::picker(select)]:text-foreground \
     [&::picker(select)]:outline-none [&::picker-icon]:hidden",
);

/// The icon marking the checked option.
const CHECKMARK: IconData = iconify_icon!("lucide:check");

/// Supplies the checkmark icon as a data URI in `--select-checkmark`, so the
/// options' `::checkmark` can use it as a mask.
fn checkmark_style(cx: &Cx) -> String {
    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}">{}</svg>"#,
        CHECKMARK.view_box(),
        CHECKMARK.into_body().render(cx),
    );
    let mut style = String::from(r#"--select-checkmark: url("data:image/svg+xml,"#);
    // Percent-encode the characters that cannot appear in a double-quoted
    // CSS url().
    for char in svg.chars() {
        match char {
            '%' => style.push_str("%25"),
            '"' => style.push_str("%22"),
            '#' => style.push_str("%23"),
            _ => style.push(char),
        }
    }
    style.push_str(r#"")"#);
    style
}

/// A styled native `<select>`.
///
/// Browsers that support customizable selects (`appearance: base-select`)
/// get the neobrutalism trigger and a styled picker with checkmarks; others
/// keep their native picker under the same trigger. Keyboard handling, form
/// submission and validation are the browser's own.
///
/// Pass options as children, usually inside a [`select_content`]. Set
/// `placeholder` to show a hint until something is chosen (an empty-valued,
/// hidden option, so `required` rejects it). Classes in `attrs` apply to the
/// wrapper, so size the control there (e.g. `w-[180px]`); every other
/// attribute (`name`, `id`, `disabled=""`, `required=""`, `@change`) goes on
/// the `<select>`.
///
/// ```ignore
/// view! {
///     select(
///         placeholder: "Select a fruit",
///         attrs: attributes! { name="fruit" class="w-[180px]" },
///         select_content(
///             select_group(attrs: attributes! { label="Fruits" },
///                 select_label("Fruits")
///                 select_item(attrs: attributes! { value="apple" }, "Apple")
///                 select_item(attrs: attributes! { value="banana" }, "Banana")
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn select(
    cx: &Cx,
    /// Text shown until an option is chosen. `#[into]` lets callers pass a
    /// plain `"text"`, which converts to `Some("text")`.
    #[into]
    #[default]
    placeholder: Option<&str>,
    /// Extra attributes for the `<select>`; classes go on the wrapper.
    #[default]
    mut attrs: Attributes,
    /// The options.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    // `appearance: base-select` is set from the wrapper so it outranks the
    // `appearance-none` fallback regardless of stylesheet order; browsers
    // without support drop it and keep the fallback.
    Ok(view! {
        <span
            data-slot="select"
            class=(class!(
                "relative block [&>select]:[appearance:base-select] \
                 [&:has(select:open)>svg]:rotate-180 has-[select:disabled]:[&>svg]:opacity-50",
                attrs.remove("class"),
            ))
            style=(checkmark_style(cx))
        >
            <select data-slot="select-trigger" class=(class!(TRIGGER, PICKER)) (attrs)>
                if let Some(placeholder) = placeholder {
                    <option value="" disabled="" selected="" hidden="">(placeholder)</option>
                }
                (child)
            </select>
            icon(
                data: iconify_icon!("lucide:chevron-down"),
                attrs: attributes! {
                    class="pointer-events-none absolute top-1/2 right-3 size-4 -translate-y-1/2 \
                        text-main-foreground transition-transform"
                }
            )
        </span>
    })
}

/// A custom closed-state button for a [`select`], placed first among its
/// children.
///
/// Optional, and only used by browsers with customizable selects: it lets the
/// closed control show the chosen option's full content, icons included, via
/// a [`select_value`]. Other browsers ignore it and show the option's text.
#[component]
#[allow(dead_code)]
pub async fn select_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            data-slot="select-button"
            class=(class!(
                "flex min-w-0 flex-1 items-center gap-2 border-0 bg-transparent p-0 text-left \
                 text-inherit",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </button>
    })
}

/// The chosen option's content, mirrored into a [`select_trigger`] by the
/// browser (a `<selectedcontent>` element).
#[component]
#[allow(dead_code)]
pub async fn select_value(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <selectedcontent
            data-slot="select-value"
            class=(class!("flex flex-1 items-center gap-2 text-left", attrs.remove("class")))
            (attrs)
        ></selectedcontent>
    })
}

/// The list of options inside a [`select`], padded inside the picker.
#[component]
pub async fn select_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="select-content" class=(class!("p-1", attrs.remove("class"))) (attrs)>
            (child)
        </div>
    })
}

/// A group of related options, rendered as an `<optgroup>`.
///
/// Give it a `label` attribute through `attrs` for native pickers, and a
/// [`select_label`] first inside it for the styled picker.
#[component]
pub async fn select_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <optgroup data-slot="select-group" class=(attrs.remove("class")) (attrs)>
            (child)
        </optgroup>
    })
}

/// The heading of a [`select_group`] in the styled picker, rendered as a
/// `<legend>`.
#[component]
pub async fn select_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <legend
            data-slot="select-label"
            class=(class!(
                "border-2 border-transparent py-1.5 pr-8 pl-2 text-sm font-base text-foreground",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </legend>
    })
}

/// Classes for an option and its hover, focus and disabled states, with the
/// checkmark of the chosen option at the right edge.
///
/// Octane's `focus:` and `data-highlighted:` states become `focus-visible:`:
/// the browser focuses the first (or chosen) option whenever the picker
/// opens, even by mouse, where Base UI highlights nothing until the pointer
/// or the keyboard moves.
const ITEM: StaticClass = class!(
    "relative flex w-full cursor-default select-none items-center gap-2 rounded-base py-1.5 \
     pr-8 pl-2 text-sm border-2 border-transparent font-base outline-none \
     hover:bg-main hover:text-main-foreground hover:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
     disabled:pointer-events-none disabled:opacity-50 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
     [&::checkmark]:absolute [&::checkmark]:right-2 [&::checkmark]:size-4 \
     [&::checkmark]:content-[''] [&::checkmark]:bg-current \
     [&::checkmark]:[mask-size:100%_100%] [&::checkmark]:[mask-image:var(--select-checkmark)]",
);

/// One choice of a [`select`], rendered as an `<option>`.
///
/// Pass `value` (and `selected=""` or `disabled=""`) through `attrs`. The
/// content may include an icon, shown by browsers with customizable selects.
#[component]
pub async fn select_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <option data-slot="select-item" class=(class!(ITEM, attrs.remove("class"))) (attrs)>
            <span
                data-slot="select-item-text"
                class="flex flex-1 items-center gap-2 whitespace-nowrap"
            >
                (child)
            </span>
        </option>
    })
}

/// A thin rule between options, rendered as an `<hr>`.
#[component]
pub async fn select_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <hr
            data-slot="select-separator"
            class=(class!("-mx-1 my-1 h-px border-0 bg-border", attrs.remove("class")))
            (attrs)
        >
    })
}

/// Classes for a scroll hint pinned to an edge of the picker.
const SCROLL_HINT: StaticClass = class!(
    "sticky z-10 flex w-full cursor-default items-center justify-center bg-background py-1 \
     font-base text-foreground",
);

/// A decorative arrow pinned to the top of a long picker.
///
/// The picker scrolls natively, so this is a visual hint only, hidden from
/// assistive technology. Place it first inside a [`select_content`].
#[component]
#[allow(dead_code)]
pub async fn select_scroll_up_button(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="select-scroll-up"
            aria-hidden="true"
            class=(class!(SCROLL_HINT, "top-0", attrs.remove("class")))
            (attrs)
        >
            icon(
                data: iconify_icon!("lucide:chevron-up"),
                attrs: attributes! { class="size-4" }
            )
        </div>
    })
}

/// A decorative arrow pinned to the bottom of a long picker.
///
/// The picker scrolls natively, so this is a visual hint only, hidden from
/// assistive technology. Place it last inside a [`select_content`].
#[component]
#[allow(dead_code)]
pub async fn select_scroll_down_button(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="select-scroll-down"
            aria-hidden="true"
            class=(class!(SCROLL_HINT, "bottom-0", attrs.remove("class")))
            (attrs)
        >
            icon(
                data: iconify_icon!("lucide:chevron-down"),
                attrs: attributes! { class="size-4" }
            )
        </div>
    })
}
