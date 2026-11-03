//! A command menu: a search box over a filtered list of items, in place of
//! cmdk.
//!
//! The markup follows cmdk's, including its `cmdk-*` attributes, so the class
//! strings that style `[cmdk-group-heading]` or `[cmdk-item]` work unchanged.
//! The behavior is one small inline script on the [`command`] root:
//!
//! - Typing in the [`command_input`] hides the items whose `value` and text
//!   don't contain every word typed, hides groups left empty and separators
//!   while a search is active, and shows [`command_empty`] when nothing
//!   matches.
//! - ArrowUp / ArrowDown (and Home / End) move the highlight, marked with
//!   `data-selected="true"` and `aria-selected="true"`; the pointer moves it
//!   too. Enter clicks the highlighted item, so an item "selects" through its
//!   ordinary click handler.
//!
//! Unlike cmdk there is no fuzzy ranking: matching items keep their order.
//! The input does not set `aria-activedescendant`.
//!
//! [`command_dialog`] puts a command inside a [`dialog`](super::dialog::dialog).

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::dialog::{dialog, dialog_content, dialog_description, dialog_header, dialog_title};

/// The [`command`] root's `oninput` handler. The root's other event handlers
/// call it too (`this.oninput(event)`), so one script sees every event:
/// `input` from the search box, `keydown`, `pointermove` and `focusin`.
///
/// Other components drive a command the same way: a combobox calls
/// `command.oninput(event)` with events from an input outside the command.
pub const COMMAND_SCRIPT: &str = r#"
const items = [...this.querySelectorAll('[data-slot=command-item]')];
const usable = () => items.filter((i) => !i.hidden && i.dataset.disabled !== 'true');
const active = items.find((i) => i.dataset.selected === 'true');
// Highlights one item (or none), the way cmdk marks it.
const select = (item, scroll) => {
  for (const i of items) {
    i.dataset.selected = i === item;
    i.setAttribute('aria-selected', i === item);
  }
  if (scroll) item?.scrollIntoView({ block: 'nearest' });
};
if (event.type === 'input') {
  // Keep the items whose value or text holds every word typed.
  const words = event.target.value.toLowerCase().split(/\s+/).filter(Boolean);
  for (const i of items) {
    const text = `${i.dataset.value ?? ''} ${i.textContent}`.toLowerCase();
    i.hidden = !words.every((w) => text.includes(w));
  }
  for (const g of this.querySelectorAll('[data-slot=command-group]')) {
    g.hidden = !g.querySelector('[data-slot=command-item]:not([hidden])');
  }
  for (const s of this.querySelectorAll('[data-slot=command-separator]')) {
    s.hidden = words.length > 0;
  }
  for (const e of this.querySelectorAll('[data-slot=command-empty]')) {
    e.hidden = items.some((i) => !i.hidden);
  }
  select(usable()[0], true);
} else if (event.type === 'keydown' && !event.isComposing) {
  const list = usable();
  const at = list.indexOf(active);
  if (event.key === 'Enter' && active) {
    event.preventDefault();
    active.click();
  }
  const next = {
    ArrowDown: list[Math.min(at + 1, list.length - 1)],
    ArrowUp: list[Math.max(at - 1, 0)],
    Home: list[0],
    End: list[list.length - 1],
  }[event.key];
  if (next) {
    event.preventDefault();
    select(next, true);
  }
} else if (event.type === 'pointermove') {
  const item = event.target.closest('[data-slot=command-item]');
  if (item && item !== active && usable().includes(item)) select(item, false);
} else if (event.type === 'focusin' && !active) {
  select(usable()[0], true);
}
"#;

/// Classes for the [`command`] root.
const COMMAND: StaticClass = class!(
    "flex h-full w-full flex-col overflow-hidden rounded-[0px] border-2 border-border \
     bg-background font-base text-foreground",
);

/// A command menu: holds a [`command_input`] and a [`command_list`].
///
/// It filters and navigates its items in the browser with
/// [`COMMAND_SCRIPT`], so it needs no runtime. Give an item an `onclick` (or
/// a runtime `@click`) to act when it is chosen with a click or Enter. The
/// root uses `oninput`, `onkeydown`, `onpointermove` and `onfocusin` itself,
/// so don't pass those in `attrs`; other attributes are forwarded to the
/// `<div>`, with extra classes appended.
///
/// ```ignore
/// view! {
///     command(attrs: attributes! { class="max-w-[450px] shadow-shadow" },
///         command_input(attrs: attributes! { placeholder="Type a command or search..." })
///         command_list(
///             command_empty("No results found.")
///             command_group(heading: "Suggestions",
///                 command_item(<span>"Calendar"</span>)
///                 command_item(<span>"Search Emoji"</span>)
///             )
///             command_separator()
///             command_group(heading: "Settings",
///                 command_item(<span>"Profile"</span> command_shortcut("⌘P"))
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn command(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command"
            cmdk-root=""
            oninput=(COMMAND_SCRIPT)
            onkeydown="this.oninput(event)"
            onpointermove="this.oninput(event)"
            onfocusin="this.oninput(event)"
            class=(class!(COMMAND, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes the [`command_dialog`] gives its [`command`]: taller rows and
/// larger icons.
const DIALOG_COMMAND: &str = "**:data-[slot=command-input-wrapper]:h-12 \
    [&_[cmdk-group-heading]]:px-2 [&_[cmdk-group-heading]]:font-heading \
    [&_[cmdk-group-heading]]:mb-1 [&_[cmdk-group]]:px-2 [&_[cmdk-input-wrapper]_svg]:h-5 \
    [&_[cmdk-input-wrapper]_svg]:w-5 [&_[cmdk-input]]:h-12 [&_[cmdk-item]]:px-2 \
    [&_[cmdk-item]]:py-3 [&_[cmdk-item]_svg]:h-5 [&_[cmdk-item]_svg]:w-5";

/// A [`command`] in a modal [`dialog`]: the command palette.
///
/// Give it an `id` and open it with a
/// [`dialog_trigger`](super::dialog::dialog_trigger) naming that id. Its
/// children go inside the command, so pass the input and the list directly.
/// The dialog focuses the input when it opens. `title` and `description`
/// name the dialog for assistive technology and are not shown. `attrs` are
/// forwarded to the `<dialog>`.
///
/// ```ignore
/// view! {
///     dialog_trigger(
///         target: "palette",
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
///         },
///         "Open Menu"
///     )
///     command_dialog(id: "palette",
///         command_input(attrs: attributes! { placeholder="Type a command or search..." })
///         command_list(
///             command_empty("No results found.")
///             command_group(heading: "Suggestions", command_item("Calendar"))
///         )
///     )
/// }
/// ```
#[component]
pub async fn command_dialog(
    /// The `id` that the dialog trigger's `target` refers to.
    id: &str,
    /// The dialog's accessible title.
    #[default("Command Palette")]
    title: &str,
    /// The dialog's accessible description.
    #[default("Search for a command to run...")]
    description: &str,
    /// Extra attributes for the `<dialog>` element.
    #[default]
    attrs: Attributes,
    /// The command's parts: a [`command_input`] and a [`command_list`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let title_id = format!("{id}-title");

    Ok(view! {
        dialog(
            id: id,
            attrs: attributes! { aria-labelledby=(title_id.clone()) (attrs) },
            // `!` overrides the panel's `p-6`, `rounded-base` and `border-2`.
            dialog_content(
                attrs: attributes! {
                    class="overflow-hidden p-0! rounded-[0px]! shadow-shadow border-0!"
                },
                dialog_header(attrs: attributes! { class="sr-only" },
                    dialog_title(attrs: attributes! { id=(title_id) }, (title))
                    dialog_description((description))
                )
                command(attrs: attributes! { class=(DIALOG_COMMAND) }, (child))
            )
        )
    })
}

/// The search box of a [`command`], with a search icon in front.
///
/// `attrs` are forwarded to the `<input>` (`placeholder`, `autofocus`, ...),
/// with extra classes appended. Inside a popover, add `autofocus=""` so the
/// box takes focus when the popover opens.
#[component]
pub async fn command_input(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-input-wrapper"
            class="flex h-9 gap-2 items-center border-b-2 border-border px-3"
        >
            icon(data: iconify_icon!("lucide:search"), attrs: attributes! { class="size-4 shrink-0" })
            <input
                data-slot="command-input"
                cmdk-input=""
                type="text"
                role="combobox"
                aria-autocomplete="list"
                aria-expanded="true"
                autocomplete="off"
                autocorrect="off"
                spellcheck="false"
                class=(class!(
                    "flex h-10 w-full rounded-base bg-transparent py-3 text-sm outline-hidden \
                     placeholder:text-foreground placeholder:opacity-50 \
                     disabled:cursor-not-allowed disabled:opacity-50",
                    attrs.remove("class"),
                ))
                (attrs)
            >
        </div>
    })
}

/// The scrolling list of a [`command`]'s groups and items.
#[component]
pub async fn command_list(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-list"
            cmdk-list=""
            role="listbox"
            class=(class!(
                "max-h-[300px] scroll-py-1 overflow-x-hidden overflow-y-auto",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The message a [`command_list`] shows when no item matches the search.
/// It is hidden until a search leaves no item visible.
#[component]
pub async fn command_empty(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-empty"
            cmdk-empty=""
            role="presentation"
            hidden=""
            class=(class!("py-6 text-center text-sm", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A labelled group of [`command_item`]s. It hides itself while a search
/// leaves none of its items visible.
#[component]
pub async fn command_group(
    /// The group's heading, shown above its items.
    #[into]
    #[default]
    heading: Option<&str>,
    #[default] mut attrs: Attributes,
    /// The group's items.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-group"
            cmdk-group=""
            role="presentation"
            class=(class!(
                "text-foreground overflow-hidden p-2 [&_[cmdk-group-heading]]:px-2 \
                 [&_[cmdk-group-heading]]:py-1.5 [&_[cmdk-group-heading]]:text-base \
                 [&_[cmdk-group-heading]]:font-heading",
                attrs.remove("class"),
            ))
            (attrs)
        >
            if let Some(heading) = heading {
                <div cmdk-group-heading="" aria-hidden="true">(heading)</div>
            }
            <div cmdk-group-items="" role="group" aria-label=(heading)>(child)</div>
        </div>
    })
}

/// A line between groups. It hides itself while a search is active.
#[component]
pub async fn command_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-separator"
            cmdk-separator=""
            role="separator"
            class=(class!("-mx-1 h-0.5 bg-border", attrs.remove("class")))
            (attrs)
        ></div>
    })
}

/// Classes for a [`command_item`], highlighted through `aria-selected`.
const ITEM: StaticClass = class!(
    "relative flex cursor-default select-none items-center rounded-base px-2 py-1.5 gap-2 \
     text-sm text-foreground border-2 border-transparent aria-selected:border-border \
     aria-selected:bg-main aria-selected:text-main-foreground \
     data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-50 \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
);

/// One choice in a [`command`].
///
/// The search matches its `value` (when given) and its text. A click, or
/// Enter while it is highlighted, fires its `click` event: put an `onclick`
/// or a runtime `@click` in `attrs` to act on it. A `disabled` item stays
/// visible but can't be highlighted or chosen.
#[component]
pub async fn command_item(
    /// The text the search matches besides the item's own text, such as an id.
    #[into]
    #[default]
    value: Option<&str>,
    /// Whether the item can't be chosen.
    #[default]
    disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="command-item"
            cmdk-item=""
            role="option"
            data-value=(value)
            data-selected="false"
            aria-selected="false"
            data-disabled=(if disabled { "true" } else { "false" })
            aria-disabled=(disabled.then_some("true"))
            class=(class!(ITEM, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A keyboard shortcut shown at the end of a [`command_item`].
#[component]
pub async fn command_shortcut(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="command-shortcut"
            class=(class!("ml-auto text-xs tracking-widest", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}
