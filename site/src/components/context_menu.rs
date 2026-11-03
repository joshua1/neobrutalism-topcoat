// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// Which way a [`context_menu_content`] opens from the pointer.
///
/// [`Default`] is `ContextMenuSide::Bottom`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ContextMenuSide {
    /// Above the pointer.
    Top,
    /// To the right of the pointer, its top edge at the pointer.
    Right,
    /// Below and to the right of the pointer.
    #[default]
    Bottom,
    /// To the left of the pointer.
    Left,
}

impl ContextMenuSide {
    /// The `data-side` value the open handler reads to place the menu.
    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}

/// The visual style of a [`context_menu_item`].
///
/// [`Default`] is `ContextMenuItemVariant::Default`, used when no variant is
/// given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ContextMenuItemVariant {
    /// A regular action.
    #[default]
    Default,
    /// A dangerous action, such as deleting, shown in red.
    Destructive,
}

impl ContextMenuItemVariant {
    /// The `data-variant` value the item's classes key off.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Destructive => "destructive",
        }
    }
}

/// Opens the menu at the pointer instead of the browser's own context menu,
/// keeping it inside the viewport.
const OPEN_JS: &str = "event.preventDefault();\
     const m=this.parentElement.querySelector(':scope>[data-slot=context-menu-content]');\
     if(!m)return;if(!m.matches(':popover-open'))m.showPopover();\
     const s=m.dataset.side,w=m.offsetWidth,h=m.offsetHeight;\
     let x=event.clientX,y=event.clientY;if(s==='top')y-=h;if(s==='left')x-=w;\
     m.style.left=Math.max(0,Math.min(x,innerWidth-w))+'px';\
     m.style.top=Math.max(0,Math.min(y,innerHeight-h))+'px';m.focus()";

/// Closes open submenus when the menu closes, so it reopens collapsed.
const RESET_JS: &str =
    "if(event.newState==='closed')this.querySelectorAll('details[open]').forEach(d=>d.open=false)";

/// Closes the menu after an item is chosen.
const CLOSE_JS: &str = "this.closest('[popover]')?.hidePopover()";

/// A menu that opens at the pointer when its trigger area is right-clicked.
///
/// Pass a [`context_menu_trigger`] and a [`context_menu_content`] as children.
/// The panel is a native `popover`, so a press outside it or Escape closes it,
/// and it renders above everything else. An inline `contextmenu` handler on
/// the trigger opens it at the pointer. Touch devices that fire `contextmenu`
/// on a long press (Android) open it too; iOS Safari does not fire it. Items
/// use normal Tab navigation, without the ARIA menu pattern's arrow keys.
///
/// The wrapper is `display: contents`, so it does not affect layout. `attrs`
/// are forwarded to it.
///
/// ```ignore
/// view! {
///     context_menu(
///         context_menu_trigger(
///             attrs: attributes! { class="flex h-[150px] w-[300px] items-center justify-center" },
///             "Right click here"
///         )
///         context_menu_content(
///             context_menu_item("Back" context_menu_shortcut("⌘["))
///             context_menu_item(variant: ContextMenuItemVariant::Destructive, "Delete")
///         )
///     )
/// }
/// ```
#[component]
pub async fn context_menu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu"
            class=(class!("contents", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The area that opens the [`context_menu`] when right-clicked.
///
/// `attrs` are forwarded to its `<div>`, with extra classes appended to its
/// classes.
#[component]
pub async fn context_menu_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu-trigger"
            oncontextmenu=(OPEN_JS)
            class=(class!("select-none", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A labelled set of related items, rendered as a `<div role="group">`.
#[component]
pub async fn context_menu_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="context-menu-group" role="group" class=(attrs.remove("class")) (attrs)>
            (child)
        </div>
    })
}

/// Renders its children in place.
///
/// The menu is a popover in the browser's top layer already, so there is
/// nothing to move. This part only exists to keep ported markup compiling.
#[component]
#[allow(dead_code)]
pub async fn context_menu_portal(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// Classes for a floating menu panel. The entry fade uses `@starting-style`.
const POPUP: StaticClass = class!(
    "z-50 min-w-[8rem] overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     font-base text-foreground outline-none transition-opacity duration-150 starting:opacity-0",
);

/// The menu panel of a [`context_menu`], shown at the pointer.
///
/// `side` picks which way it opens from the pointer (default `Bottom`).
/// `attrs` are forwarded to the popover `<div>`, with extra classes appended
/// to its classes; set a width such as `w-64` there.
#[component]
pub async fn context_menu_content(
    /// Which way the menu opens from the pointer.
    #[default]
    side: ContextMenuSide,
    /// Extra attributes for the popover `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The menu's items.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            popover="auto"
            data-slot="context-menu-content"
            data-side=(side.as_str())
            role="menu"
            tabindex="-1"
            ontoggle=(RESET_JS)
            class=(class!(POPUP, "fixed inset-auto m-0", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for an action row and its hover, focus and disabled states.
const ITEM: StaticClass = class!(
    "relative flex cursor-default select-none items-center rounded-base border-2 \
     border-transparent px-2 py-1.5 gap-2 text-sm outline-none \
     hover:bg-main hover:text-main-foreground hover:border-border \
     focus:bg-main focus:text-main-foreground focus:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
     disabled:pointer-events-none disabled:opacity-50 data-inset:pl-8 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
     w-full text-left",
);

/// Classes for the destructive variant, keyed off `data-variant`.
const DESTRUCTIVE: StaticClass = class!(
    "data-[variant=destructive]:text-red-500 data-[variant=destructive]:hover:bg-red-500 \
     data-[variant=destructive]:hover:text-white data-[variant=destructive]:focus:bg-red-500 \
     data-[variant=destructive]:focus:text-white \
     data-[variant=destructive]:focus-visible:bg-red-500 \
     data-[variant=destructive]:focus-visible:text-white",
);

/// One action in a [`context_menu_content`], rendered as a `<button>`.
///
/// Choosing it closes the menu unless `close_on_click` is `false`. `variant`
/// picks the regular or destructive look. Set `inset` to indent its text in
/// line with checkbox and radio items. Add `disabled=""` and handlers such as
/// `@click` through `attrs`.
#[component]
pub async fn context_menu_item(
    /// Indents the item to line up with items that have an indicator.
    #[default]
    inset: bool,
    /// The regular or destructive look.
    #[default]
    variant: ContextMenuItemVariant,
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
            data-slot="context-menu-item"
            data-inset=(inset)
            data-variant=(variant.as_str())
            onclick=(close_on_click.then_some(CLOSE_JS))
            class=(class!(ITEM, DESTRUCTIVE, attrs.remove("class")))
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
     border-transparent py-1.5 pl-8 pr-2 gap-2 text-sm font-base outline-hidden \
     hover:bg-main hover:text-main-foreground hover:border-border \
     has-focus-visible:bg-main has-focus-visible:text-main-foreground \
     has-focus-visible:border-border has-disabled:pointer-events-none has-disabled:opacity-50",
);

/// Classes for the box holding an indicator, shown only while the input
/// before it is checked.
const INDICATOR: StaticClass = class!(
    "pointer-events-none absolute left-2 flex h-3.5 w-3.5 items-center justify-center \
     invisible peer-checked:visible",
);

/// An item that turns an option on or off, backed by a native checkbox.
///
/// Choosing it leaves the menu open. Classes in `attrs` apply to the row's
/// `<label>`; every other attribute (`checked=""`, `name`, `@change`) goes on
/// the `<input>`.
#[component]
pub async fn context_menu_checkbox_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="context-menu-checkbox-item"
            class=(class!(CHOICE_ITEM, attrs.remove("class")))
        >
            <input type="checkbox" role="menuitemcheckbox" class="peer sr-only" (attrs)>
            <span class=(INDICATOR)>
                icon(
                    data: iconify_icon!("lucide:check"),
                    attrs: attributes! { class="h-4 w-4" }
                )
            </span>
            (child)
        </label>
    })
}

/// A set of [`context_menu_radio_item`]s of which one can be chosen.
///
/// Give every item in the group the same `name` attribute.
#[component]
pub async fn context_menu_radio_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu-radio-group"
            role="group"
            class=(attrs.remove("class"))
            (attrs)
        >
            (child)
        </div>
    })
}

/// An item that picks one option of a [`context_menu_radio_group`], backed by
/// a native radio button.
///
/// Classes in `attrs` apply to the row's `<label>`; every other attribute goes
/// on the `<input>`. Give the items of one group the same `name`, a `value`
/// each, and `checked=""` on the selected one.
#[component]
pub async fn context_menu_radio_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="context-menu-radio-item"
            class=(class!(CHOICE_ITEM, attrs.remove("class")))
        >
            <input type="radio" role="menuitemradio" class="peer sr-only" (attrs)>
            <span class=(INDICATOR)>
                icon(
                    data: iconify_icon!("lucide:circle"),
                    attrs: attributes! { class="h-2 w-2 fill-current [&_*]:fill-current" }
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
pub async fn context_menu_label(
    /// Indents the label to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu-label"
            data-inset=(inset)
            class=(class!(
                "px-2 py-1.5 text-sm font-base border-2 border-transparent data-inset:pl-8",
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
pub async fn context_menu_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu-separator"
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
pub async fn context_menu_shortcut(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="context-menu-shortcut"
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

/// Classes that hide the native disclosure marker of a `<summary>`.
const SUMMARY: StaticClass = class!("list-none [&::-webkit-details-marker]:hidden");

/// A nested menu opened from a row of its parent menu.
///
/// A native `<details>` element: its trigger row toggles it on click or Enter.
/// It does not open on hover. Closing the parent menu also closes it.
///
/// ```ignore
/// view! {
///     context_menu_sub(
///         context_menu_sub_trigger("More Tools")
///         context_menu_sub_content(
///             context_menu_item("Save Page...")
///         )
///     )
/// }
/// ```
#[component]
pub async fn context_menu_sub(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="context-menu-sub"
            class=(class!(
                "group/context-menu-sub [anchor-scope:--context-menu-sub]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </details>
    })
}

/// The row that opens or closes a [`context_menu_sub`], with a chevron
/// pointing at the submenu. It stays highlighted while the submenu is open.
#[component]
pub async fn context_menu_sub_trigger(
    /// Indents the row to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="context-menu-sub-trigger"
            data-inset=(inset)
            aria-haspopup="menu"
            class=(class!(
                "flex cursor-default select-none items-center rounded-base border-2 \
                 border-transparent px-2 py-1.5 text-sm font-base outline-hidden data-inset:pl-8 \
                 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
                 hover:bg-main hover:text-main-foreground hover:border-border \
                 focus:bg-main focus:text-main-foreground focus:border-border \
                 focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
                 group-open/context-menu-sub:bg-main group-open/context-menu-sub:text-main-foreground \
                 group-open/context-menu-sub:border-border",
                SUMMARY,
                "[anchor-name:--context-menu-sub]",
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

/// The panel of a [`context_menu_sub`], opening to the right of its trigger
/// row (or to the left when there is no room).
///
/// It is placed with CSS anchor positioning. Browsers without it show the
/// panel just below the trigger row.
#[component]
pub async fn context_menu_sub_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="context-menu-sub-content"
            role="menu"
            tabindex="-1"
            class=(class!(
                POPUP,
                "fixed [position-anchor:--context-menu-sub] [position-area:right_span-bottom] \
                 [position-try-fallbacks:flip-inline] -mt-1.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
