// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// Which side of the trigger a [`dropdown_menu_content`] opens on.
///
/// [`Default`] is `DropdownMenuSide::Bottom`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DropdownMenuSide {
    /// Above the trigger.
    Top,
    /// To the right of the trigger.
    Right,
    /// Below the trigger.
    #[default]
    Bottom,
    /// To the left of the trigger.
    Left,
}

/// How a [`dropdown_menu_content`] lines up with its trigger along the side it
/// opens on.
///
/// [`Default`] is `DropdownMenuAlign::Center`, used when no alignment is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DropdownMenuAlign {
    /// Lines up with the trigger's start edge (left, or top for side menus).
    Start,
    /// Centers on the trigger.
    #[default]
    Center,
    /// Lines up with the trigger's end edge (right, or bottom for side menus).
    End,
}

/// CSS anchor-positioning classes placing the panel next to its trigger.
///
/// The gap is a margin on the side facing the trigger, and the panel flips to
/// the opposite side when it would overflow the viewport.
fn position(side: DropdownMenuSide, align: DropdownMenuAlign) -> StaticClass {
    use DropdownMenuAlign::{Center, End, Start};
    use DropdownMenuSide::{Bottom, Left, Right, Top};

    match (side, align) {
        (Bottom, Start) => class!("mt-1 [position-area:bottom_span-right] [position-try-fallbacks:flip-block]"),
        (Bottom, Center) => class!("mt-1 [position-area:bottom_span-all] [position-try-fallbacks:flip-block]"),
        (Bottom, End) => class!("mt-1 [position-area:bottom_span-left] [position-try-fallbacks:flip-block]"),
        (Top, Start) => class!("mb-1 [position-area:top_span-right] [position-try-fallbacks:flip-block]"),
        (Top, Center) => class!("mb-1 [position-area:top_span-all] [position-try-fallbacks:flip-block]"),
        (Top, End) => class!("mb-1 [position-area:top_span-left] [position-try-fallbacks:flip-block]"),
        (Right, Start) => class!("ml-1 [position-area:right_span-bottom] [position-try-fallbacks:flip-inline]"),
        (Right, Center) => class!("ml-1 [position-area:right_span-all] [position-try-fallbacks:flip-inline]"),
        (Right, End) => class!("ml-1 [position-area:right_span-top] [position-try-fallbacks:flip-inline]"),
        (Left, Start) => class!("mr-1 [position-area:left_span-bottom] [position-try-fallbacks:flip-inline]"),
        (Left, Center) => class!("mr-1 [position-area:left_span-all] [position-try-fallbacks:flip-inline]"),
        (Left, End) => class!("mr-1 [position-area:left_span-top] [position-try-fallbacks:flip-inline]"),
    }
}

/// Runs when the menu opens or closes. While open, a press outside the menu
/// closes it; on close, any open submenu closes too, so the menu reopens
/// collapsed.
const DISMISS_JS: &str = "if(this.open){const d=this,f=e=>{if(!d.contains(e.target)){d.open=false;\
     document.removeEventListener('pointerdown',f)}};document.addEventListener('pointerdown',f)}\
     else this.querySelectorAll('details[open]').forEach(s=>s.open=false)";

/// Closes the menu on Escape and returns focus to its trigger.
const ESCAPE_JS: &str =
    "if(event.key==='Escape'&&this.open){this.open=false;this.querySelector('summary').focus()}";

/// Closes the menu after an item is chosen and returns focus to its trigger.
const CLOSE_JS: &str = "const d=this.closest('[data-slot=dropdown-menu]');\
     if(d){d.open=false;d.querySelector('summary').focus()}";

/// A menu of actions that opens from a trigger.
///
/// Built on a native `<details>` element, so the trigger opens and closes it
/// without a script library. Small inline handlers add what `<details>` lacks:
/// a press outside the menu or Escape closes it, and choosing an item closes it.
/// Items use normal Tab navigation; the ARIA menu pattern's arrow-key
/// navigation and typeahead are not implemented.
///
/// The panel is placed with CSS anchor positioning and `position: fixed`, so it
/// is not clipped by a scrolling or `overflow-hidden` ancestor (such as a table
/// container). Browsers without anchor positioning show it just below the
/// trigger. An ancestor with a `transform` or `filter` traps fixed elements, so
/// avoid placing the menu inside one.
///
/// Pass a [`dropdown_menu_trigger`] and a [`dropdown_menu_content`] as
/// children. `attrs` are forwarded to the `<details>`, with extra classes
/// appended to its classes. Add `open=""` to render it open.
///
/// ```ignore
/// view! {
///     dropdown_menu(
///         dropdown_menu_trigger(
///             attrs: attributes! {
///                 class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
///             },
///             "Open"
///         )
///         dropdown_menu_content(
///             align: DropdownMenuAlign::End,
///             attrs: attributes! { class="w-56" },
///             dropdown_menu_label("My Account")
///             dropdown_menu_separator()
///             dropdown_menu_item("Profile" dropdown_menu_shortcut("⇧⌘P"))
///             dropdown_menu_item(attrs: attributes! { disabled="" }, "API")
///         )
///     )
/// }
/// ```
#[component]
pub async fn dropdown_menu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="dropdown-menu"
            ontoggle=(DISMISS_JS)
            onkeydown=(ESCAPE_JS)
            // Any `translate` on the root (a button group's
            // `*:hover:translate-x-0!`) would make it the fixed panel's
            // containing block and squeeze the panel onto the trigger;
            // `open:hover:translate-none!` outranks such child selectors.
            class=(class!(
                "group/dropdown-menu relative inline-block [anchor-scope:--dropdown-menu] \
                 open:hover:translate-none!",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </details>
    })
}

/// Classes that hide the native disclosure marker of a `<summary>`.
const SUMMARY: StaticClass = class!("list-none [&::-webkit-details-marker]:hidden");

/// The control that opens and closes a [`dropdown_menu`], rendered as a
/// `<summary>`.
///
/// It has no styling of its own. To make it look like a button, pass
/// [`button_variants`](super::button::button_variants) classes through `attrs`.
/// Use `group-open/dropdown-menu:` classes to style its children while the menu
/// is open.
///
/// ```ignore
/// view! {
///     dropdown_menu_trigger(
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Neutral, ButtonSize::Icon))
///             aria-label="More options"
///         },
///         icon(data: iconify_icon!("lucide:ellipsis"))
///     )
/// }
/// ```
#[component]
pub async fn dropdown_menu_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="dropdown-menu-trigger"
            aria-haspopup="menu"
            class=(class!(
                SUMMARY,
                "[anchor-name:--dropdown-menu]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </summary>
    })
}

/// Classes for a floating menu panel. The entry fade uses `@starting-style`.
const POPUP: StaticClass = class!(
    "z-50 min-w-[8rem] overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     font-base text-foreground outline-none \
     fixed transition-opacity duration-150 starting:opacity-0",
);

/// The floating panel of a [`dropdown_menu`], holding its items.
///
/// `side` picks the side of the trigger it opens on (default `Bottom`) and
/// `align` how it lines up with the trigger (default `Center`). `attrs` are
/// forwarded to the panel's `<div>`, with extra classes appended to its
/// classes; set a width such as `w-56` there.
#[component]
pub async fn dropdown_menu_content(
    /// The side of the trigger the panel opens on.
    #[default]
    side: DropdownMenuSide,
    /// How the panel lines up with the trigger.
    #[default]
    align: DropdownMenuAlign,
    /// Extra attributes for the panel's `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The menu's items.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dropdown-menu-content"
            role="menu"
            tabindex="-1"
            class=(class!(
                POPUP,
                "[position-anchor:--dropdown-menu]",
                position(side, align),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A labelled set of related items, rendered as a `<div role="group">`.
#[component]
pub async fn dropdown_menu_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="dropdown-menu-group" role="group" class=(attrs.remove("class")) (attrs)>
            (child)
        </div>
    })
}

/// Renders its children in place.
///
/// Base UI moves menus to the end of `<body>`. Here the panel already escapes
/// its container through fixed positioning, so this part only exists to keep
/// ported markup compiling.
#[component]
#[allow(dead_code)]
pub async fn dropdown_menu_portal(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// Classes for an action row and its hover, focus and disabled states.
const ITEM: StaticClass = class!(
    "relative gap-2 [&_svg]:pointer-events-none [&_svg]:w-4 [&_svg]:h-4 [&_svg]:shrink-0 \
     flex cursor-default select-none items-center rounded-base border-2 border-transparent \
     data-inset:pl-8 px-2 py-1.5 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border \
     focus:bg-main focus:text-main-foreground focus:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
     disabled:pointer-events-none disabled:opacity-50 w-full text-left",
);

/// One action in a [`dropdown_menu_content`], rendered as a `<button>`.
///
/// Choosing it closes the menu unless `close_on_click` is `false`. Set `inset`
/// to indent its text in line with checkbox and radio items. Add `disabled=""`
/// through `attrs` to disable it, and handlers such as `@click` there too.
#[component]
pub async fn dropdown_menu_item(
    /// Indents the item to line up with items that have an indicator.
    #[default]
    inset: bool,
    /// Whether choosing the item closes the menu.
    #[default(true)]
    close_on_click: bool,
    /// Extra attributes for the `<button>`.
    #[default]
    mut attrs: Attributes,
    /// The item's content: an optional icon, its label and a shortcut.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            role="menuitem"
            data-slot="dropdown-menu-item"
            data-inset=(inset)
            onclick=(close_on_click.then_some(CLOSE_JS))
            class=(class!(ITEM, attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// Classes for a checkbox or radio row. The row is a `<label>`; its state
/// comes from the visually hidden input inside it.
const CHOICE_ITEM: StaticClass = class!(
    "relative flex cursor-default select-none items-center rounded-base border-2 \
     border-transparent gap-2 py-1.5 pl-8 pr-2 text-sm font-base outline-hidden transition-colors \
     hover:bg-main hover:text-main-foreground hover:border-border \
     has-focus-visible:bg-main has-focus-visible:text-main-foreground \
     has-focus-visible:border-border has-disabled:pointer-events-none has-disabled:opacity-50",
);

/// Classes for the box holding a checkbox or radio indicator, shown only while
/// the input before it is checked.
const INDICATOR: StaticClass = class!(
    "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center \
     invisible peer-checked:visible",
);

/// An item that turns an option on or off, backed by a native checkbox.
///
/// The checkbox keeps its own state and submits with a surrounding form, and
/// choosing it leaves the menu open. Classes in `attrs` apply to the row's
/// `<label>`; every other attribute (`checked=""`, `disabled=""`, `name`,
/// `@change`, `:checked`) goes on the `<input>`.
///
/// ```ignore
/// view! {
///     dropdown_menu_checkbox_item(
///         attrs: attributes! { name="status_bar" checked="" },
///         "Status Bar"
///     )
/// }
/// ```
#[component]
pub async fn dropdown_menu_checkbox_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="dropdown-menu-checkbox-item"
            class=(class!(
                CHOICE_ITEM,
                "[&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                attrs.remove("class"),
            ))
        >
            <input type="checkbox" role="menuitemcheckbox" class="peer sr-only" (attrs)>
            <span class=(INDICATOR)>
                icon(data: iconify_icon!("lucide:check"))
            </span>
            (child)
        </label>
    })
}

/// A set of [`dropdown_menu_radio_item`]s of which one can be chosen.
///
/// Native radio buttons form a group through a shared `name`, so give every
/// item in the group the same `name` attribute.
#[component]
pub async fn dropdown_menu_radio_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dropdown-menu-radio-group"
            role="group"
            class=(attrs.remove("class"))
            (attrs)
        >
            (child)
        </div>
    })
}

/// An item that picks one option of a [`dropdown_menu_radio_group`], backed by
/// a native radio button.
///
/// Classes in `attrs` apply to the row's `<label>`; every other attribute goes
/// on the `<input>`. Give the items of one group the same `name`, a `value`
/// each, and `checked=""` on the selected one.
///
/// ```ignore
/// view! {
///     dropdown_menu_radio_item(
///         attrs: attributes! { name="position" value="top" checked="" },
///         "Top"
///     )
/// }
/// ```
#[component]
pub async fn dropdown_menu_radio_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="dropdown-menu-radio-item"
            class=(class!(CHOICE_ITEM, attrs.remove("class")))
        >
            <input type="radio" role="menuitemradio" class="peer sr-only" (attrs)>
            <span class=(INDICATOR)>
                icon(
                    data: iconify_icon!("lucide:circle"),
                    attrs: attributes! { class="size-2 fill-current [&_*]:fill-current" }
                )
            </span>
            (child)
        </label>
    })
}

/// A non-interactive heading for the items after it.
///
/// Set `inset` to line it up with checkbox and radio items.
#[component]
pub async fn dropdown_menu_label(
    /// Indents the label to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dropdown-menu-label"
            data-inset=(inset)
            class=(class!(
                "px-2 py-1.5 text-sm font-heading data-inset:pl-8",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A thick rule separating groups of items.
#[component]
pub async fn dropdown_menu_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dropdown-menu-separator"
            role="separator"
            class=(class!("-mx-1 my-1 h-0.5 bg-border", attrs.remove("class")))
            (attrs)
        ></div>
    })
}

/// A keyboard shortcut hint, pushed to the end of its item.
///
/// It is only a label: register the shortcut itself in application code.
#[component]
pub async fn dropdown_menu_shortcut(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="dropdown-menu-shortcut"
            class=(class!(
                "ml-auto text-xs font-base tracking-widest",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// A nested menu opened from a row of its parent menu.
///
/// Another `<details>` element: its trigger row toggles it on click or Enter.
/// It does not open on hover. Closing the parent menu also closes it. Pass a
/// [`dropdown_menu_sub_trigger`] and a [`dropdown_menu_sub_content`] as
/// children.
///
/// ```ignore
/// view! {
///     dropdown_menu_sub(
///         dropdown_menu_sub_trigger("Invite users")
///         dropdown_menu_sub_content(
///             dropdown_menu_item("Email")
///             dropdown_menu_item("Message")
///         )
///     )
/// }
/// ```
#[component]
pub async fn dropdown_menu_sub(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="dropdown-menu-sub"
            class=(class!(
                "group/dropdown-menu-sub [anchor-scope:--dropdown-menu-sub]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The row that opens or closes a [`dropdown_menu_sub`], with a chevron
/// pointing at the submenu. It stays highlighted while the submenu is open.
#[component]
pub async fn dropdown_menu_sub_trigger(
    /// Indents the row to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="dropdown-menu-sub-trigger"
            data-inset=(inset)
            aria-haspopup="menu"
            class=(class!(
                "flex cursor-default select-none items-center rounded-base border-2 \
                 border-transparent px-2 py-1.5 text-sm font-base outline-hidden \
                 hover:bg-main hover:text-main-foreground hover:border-border \
                 focus:bg-main focus:text-main-foreground focus:border-border \
                 focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
                 group-open/dropdown-menu-sub:bg-main group-open/dropdown-menu-sub:text-main-foreground \
                 group-open/dropdown-menu-sub:border-border gap-2 data-inset:pl-8 \
                 [&_svg]:pointer-events-none [&_svg]:w-4 [&_svg]:h-4 [&_svg]:shrink-0",
                SUMMARY,
                "[anchor-name:--dropdown-menu-sub]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
            icon(
                data: iconify_icon!("lucide:chevron-right"),
                attrs: attributes! { class="ml-auto" }
            )
        </summary>
    })
}

/// The panel of a [`dropdown_menu_sub`], opening to the right of its trigger
/// row (or to the left when there is no room).
#[component]
pub async fn dropdown_menu_sub_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dropdown-menu-sub-content"
            role="menu"
            tabindex="-1"
            class=(class!(
                POPUP,
                "[position-anchor:--dropdown-menu-sub] [position-area:right_span-bottom] \
                 [position-try-fallbacks:flip-inline] -mt-1.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
