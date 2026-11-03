// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// Runs when a menu opens or closes. While open, a press outside the menu
/// closes it; on close, any open submenu closes too.
const DISMISS_JS: &str = "if(this.open){const d=this,f=e=>{if(!d.contains(e.target)){d.open=false;\
     document.removeEventListener('pointerdown',f)}};document.addEventListener('pointerdown',f)}\
     else this.querySelectorAll('details[open]').forEach(s=>s.open=false)";

/// Closes the menu on Escape and returns focus to its trigger.
const ESCAPE_JS: &str =
    "if(event.key==='Escape'&&this.open){this.open=false;this.querySelector('summary').focus()}";

/// While one menu of the bar is open, pointing at another trigger opens that
/// menu instead (the shared `name` closes the first one).
const SWITCH_JS: &str = "const d=this.parentElement;\
     if(!d.open&&d.parentElement.querySelector(':scope>details[open]'))d.open=true";

/// Closes the menu after an item is chosen and returns focus to its trigger.
const CLOSE_JS: &str = "const d=this.closest('[data-slot=menubar-menu]');\
     if(d){d.open=false;d.querySelector('summary').focus()}";

/// A horizontal bar of menus, like a desktop application's menu bar.
///
/// Pass [`menubar_menu`]s as children. Each menu is a native `<details>`
/// element sharing the `name` "menubar", so opening one closes the others.
/// Once a menu is open, pointing at another trigger switches to it. A press
/// outside or Escape closes the open menu, and choosing an item closes it.
/// The ARIA menubar pattern's arrow-key navigation and typeahead are not
/// implemented; triggers and items use normal Tab navigation.
///
/// `attrs` are forwarded to the bar's `<div>`, with extra classes appended to
/// its classes.
///
/// ```ignore
/// view! {
///     menubar(
///         menubar_menu(
///             menubar_trigger("File")
///             menubar_content(
///                 menubar_item("New Tab" menubar_shortcut("⌘T"))
///                 menubar_separator()
///                 menubar_item("Print...")
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn menubar(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="menubar"
            role="menubar"
            class=(class!(
                "flex h-11 items-center space-x-1 rounded-base border-2 border-border \
                 bg-background p-1 font-base",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One menu of a [`menubar`]: a [`menubar_trigger`] and a [`menubar_content`].
///
/// A native `<details name="menubar">`. `attrs` are forwarded to it, with
/// extra classes appended to its classes. Pass a different `name` in `attrs`
/// to stop it closing when menus of other bars open.
#[component]
pub async fn menubar_menu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            name="menubar"
            data-slot="menubar-menu"
            ontoggle=(DISMISS_JS)
            onkeydown=(ESCAPE_JS)
            class=(class!(
                "group/menubar-menu relative [anchor-scope:--menubar-menu]",
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

/// The button in the bar that opens a [`menubar_menu`], highlighted while its
/// menu is open.
#[component]
pub async fn menubar_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="menubar-trigger"
            role="menuitem"
            aria-haspopup="menu"
            onpointerenter=(SWITCH_JS)
            class=(class!(
                "flex cursor-default select-none items-center text-foreground rounded-base \
                 px-3 py-1.5 text-sm border-2 border-transparent font-heading outline-none \
                 hover:bg-main hover:text-main-foreground hover:border-border \
                 focus:bg-main focus:text-main-foreground focus:border-border \
                 focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
                 group-open/menubar-menu:bg-main group-open/menubar-menu:text-main-foreground \
                 group-open/menubar-menu:border-border",
                SUMMARY,
                "[anchor-name:--menubar-menu]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </summary>
    })
}

/// Classes for a floating menu panel, without its minimum width. The entry
/// fade uses `@starting-style`.
const POPUP: StaticClass = class!(
    "z-50 overflow-hidden rounded-base border-2 border-border bg-background p-1 \
     text-foreground outline-none fixed transition-opacity duration-150 starting:opacity-0",
);

/// The panel of a [`menubar_menu`], opening below its trigger and aligned to
/// its left edge (flipping above it when there is no room below).
///
/// It is placed with CSS anchor positioning and `position: fixed`; browsers
/// without anchor positioning show it just below the trigger. `attrs` are
/// forwarded to the panel's `<div>`, with extra classes appended to its
/// classes.
#[component]
pub async fn menubar_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="menubar-content"
            role="menu"
            tabindex="-1"
            class=(class!(
                POPUP,
                "min-w-[12rem] [position-anchor:--menubar-menu] \
                 [position-area:bottom_span-right] [position-try-fallbacks:flip-block] \
                 mt-2 -ml-1",
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
pub async fn menubar_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="menubar-group" role="group" class=(attrs.remove("class")) (attrs)>
            (child)
        </div>
    })
}

/// Renders its children in place.
///
/// The panels already escape their container through fixed positioning, so
/// this part only exists to keep ported markup compiling.
#[component]
#[allow(dead_code)]
pub async fn menubar_portal(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// A nested menu opened from a row of a menu.
///
/// Another `<details>` element: its trigger row toggles it on click or Enter.
/// It does not open on hover. Closing the parent menu also closes it.
#[component]
pub async fn menubar_sub(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <details
            data-slot="menubar-sub"
            class=(class!(
                "group/menubar-sub [anchor-scope:--menubar-sub]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </details>
    })
}

/// A set of [`menubar_radio_item`]s of which one can be chosen.
///
/// Give every item in the group the same `name` attribute.
#[component]
pub async fn menubar_radio_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="menubar-radio-group" role="group" class=(attrs.remove("class")) (attrs)>
            (child)
        </div>
    })
}

/// Classes for an action row and its hover, focus and disabled states.
const ITEM: StaticClass = class!(
    "relative flex cursor-default select-none items-center gap-2 rounded-base border-2 \
     border-transparent px-2 py-1.5 text-sm font-base outline-hidden \
     hover:bg-main hover:text-main-foreground hover:border-border \
     focus:bg-main focus:text-main-foreground focus:border-border \
     focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
     disabled:pointer-events-none disabled:opacity-50 data-inset:pl-8 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 \
     w-full text-left",
);

/// One action in a [`menubar_content`], rendered as a `<button>`.
///
/// Choosing it closes the menu unless `close_on_click` is `false`. Set `inset`
/// to indent its text in line with checkbox and radio items. Add `disabled=""`
/// and handlers such as `@click` through `attrs`.
#[component]
pub async fn menubar_item(
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
            data-slot="menubar-item"
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
     border-transparent py-1.5 pl-8 pr-2 text-sm font-base outline-hidden \
     hover:bg-main hover:text-main-foreground hover:border-border \
     has-focus-visible:bg-main has-focus-visible:text-main-foreground \
     has-focus-visible:border-border has-disabled:pointer-events-none has-disabled:opacity-50 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// Classes for the box holding an indicator, shown only while the input
/// before it is checked.
const INDICATOR: StaticClass = class!(
    "pointer-events-none absolute left-2 flex size-3.5 items-center justify-center \
     invisible peer-checked:visible",
);

/// An item that turns an option on or off, backed by a native checkbox.
///
/// Choosing it leaves the menu open. Classes in `attrs` apply to the row's
/// `<label>`; every other attribute (`checked=""`, `name`, `@change`) goes on
/// the `<input>`.
#[component]
pub async fn menubar_checkbox_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="menubar-checkbox-item"
            class=(class!(CHOICE_ITEM, attrs.remove("class")))
        >
            <input type="checkbox" role="menuitemcheckbox" class="peer sr-only" (attrs)>
            <span class=(INDICATOR)>
                icon(
                    data: iconify_icon!("lucide:check"),
                    attrs: attributes! { class="size-4" }
                )
            </span>
            (child)
        </label>
    })
}

/// An item that picks one option of a [`menubar_radio_group`], backed by a
/// native radio button.
///
/// Classes in `attrs` apply to the row's `<label>`; every other attribute goes
/// on the `<input>`. Give the items of one group the same `name`, a `value`
/// each, and `checked=""` on the selected one.
#[component]
pub async fn menubar_radio_item(
    /// Extra attributes for the `<input>`; classes go on the row.
    #[default]
    mut attrs: Attributes,
    /// The item's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="menubar-radio-item"
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
pub async fn menubar_label(
    /// Indents the label to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="menubar-label"
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
pub async fn menubar_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="menubar-separator"
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
pub async fn menubar_shortcut(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="menubar-shortcut"
            class=(class!("ml-auto text-xs tracking-widest", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The row that opens or closes a [`menubar_sub`], with a chevron pointing at
/// the submenu. It stays highlighted while the submenu is open.
#[component]
pub async fn menubar_sub_trigger(
    /// Indents the row to line up with items that have an indicator.
    #[default]
    inset: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <summary
            data-slot="menubar-sub-trigger"
            data-inset=(inset)
            aria-haspopup="menu"
            class=(class!(
                "flex cursor-default select-none items-center gap-2 rounded-base border-2 \
                 border-transparent px-3 py-1.5 text-sm font-base outline-hidden \
                 hover:bg-main hover:text-main-foreground hover:border-border \
                 focus:bg-main focus:text-main-foreground focus:border-border \
                 focus-visible:bg-main focus-visible:text-main-foreground focus-visible:border-border \
                 group-open/menubar-sub:bg-main group-open/menubar-sub:text-main-foreground \
                 group-open/menubar-sub:border-border data-inset:pl-8 \
                 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
                SUMMARY,
                "[anchor-name:--menubar-sub]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
            icon(
                data: iconify_icon!("lucide:chevron-right"),
                attrs: attributes! { class="ml-auto size-4" }
            )
        </summary>
    })
}

/// The panel of a [`menubar_sub`], opening to the right of its trigger row
/// (or to the left when there is no room).
#[component]
pub async fn menubar_sub_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="menubar-sub-content"
            role="menu"
            tabindex="-1"
            class=(class!(
                POPUP,
                "min-w-[8rem] [position-anchor:--menubar-sub] [position-area:right_span-bottom] \
                 [position-try-fallbacks:flip-inline] -mt-1.5",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
