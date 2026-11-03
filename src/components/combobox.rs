//! A combobox: a [`command`](super::command::command) list of choices in a
//! [`popover_content`](super::popover::popover_content), opened from a button
//! or typed into from an input.
//!
//! - [`combobox`] holds the parts and a hidden input with the chosen value.
//! - [`combobox_trigger`] is the button variant: a button that opens the
//!   popover, showing the choice in a [`combobox_value`]. The popover then
//!   holds a whole command, search box included.
//! - [`combobox_input`] is the autocomplete variant: an input anchored to
//!   the popover. Typing opens it and filters the command list inside it.
//! - [`combobox_item`] is a command item with a value. Clicking it (or Enter
//!   while it is highlighted) checks it, writes its value to the hidden
//!   input, shows it in the trigger or input, and closes the popover. In a
//!   `multiple` combobox it toggles instead and the popover stays open.
//!
//! All of it runs as small inline scripts, so no runtime is needed. The hidden
//! input fires a bubbling `change` event after each choice, which a runtime
//! `@change` on the [`combobox`] can listen to.
//!
//! Clicking the checked item of a single combobox unchecks it, as in the
//! Octane examples.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::command::command_item;
use super::input::input;
use super::popover::{popover_anchor, popover_trigger};

/// The `onclick` of a [`combobox_item`]: chooses the item (`this`).
const SELECT_SCRIPT: &str = r#"
const root = this.closest('[data-slot=combobox]');
const multiple = root.hasAttribute('data-multiple');
const items = [...root.querySelectorAll('[data-slot=command-item][data-value]')];
// Check this item; a single combobox unchecks the others.
const on = this.dataset.checked !== 'true';
for (const i of items) {
  if (i === this) i.dataset.checked = on;
  else if (!multiple) i.dataset.checked = false;
}
const chosen = items.filter((i) => i.dataset.checked === 'true');
const label = (i) => i.dataset.label ?? i.textContent.trim();
const hidden = root.querySelector(':scope > input[type=hidden]');
hidden.value = chosen.map((i) => i.dataset.value).join(',');
hidden.dispatchEvent(new Event('change', { bubbles: true }));
// Show the choice: a copy of the item (without its check) in the trigger, or
// the labels in an input or a multiple trigger.
const shown = root.querySelector('[data-slot=combobox-value]');
if (shown && multiple) shown.textContent = chosen.map(label).join(', ');
if (shown && !multiple) {
  const copy = chosen[0]?.cloneNode(true);
  copy?.querySelectorAll('[data-slot=combobox-item-indicator]').forEach((e) => e.remove());
  shown.replaceChildren(...(copy?.childNodes ?? []));
}
const field = root.querySelector('[data-slot=combobox-input]');
if (field) field.value = chosen.map(label).join(', ');
if (!multiple) {
  // Close the popover and clear the search for next time.
  this.closest('[popover]')?.hidePopover();
  const command = this.closest('[data-slot=command]');
  const search = command.querySelector('[data-slot=command-input]');
  if (search) search.value = '';
  command.oninput({ type: 'input', target: { value: '' } });
}
"#;

/// The `oninput` and `onkeydown` of a [`combobox_input`]: opens the popover
/// named by `aria-controls` while typing (or on ArrowDown), then hands the
/// event to the command inside it, which filters and moves the highlight.
const AUTOCOMPLETE_SCRIPT: &str = r#"
const list = document.getElementById(this.getAttribute('aria-controls'));
if (!list.matches(':popover-open') && (event.type === 'input' || event.key === 'ArrowDown')) {
  list.showPopover({ source: this });
}
if (list.matches(':popover-open')) list.querySelector('[data-slot=command]').oninput(event);
"#;

/// A combobox: groups a [`combobox_trigger`] or [`combobox_input`] with the
/// popover that lists its [`combobox_item`]s, and keeps the chosen value in a
/// hidden input named `name`.
///
/// `value` is the value the hidden input starts with; mark the matching item
/// `checked` and show it in the [`combobox_value`] too. With `multiple`, the
/// values of every checked item are joined with commas. It renders as
/// `display: contents`, so the trigger lays out as if the wrapper weren't
/// there. `attrs` are forwarded to the wrapper `<div>`.
///
/// ```ignore
/// view! {
///     combobox(name: "framework",
///         combobox_trigger(target: "frameworks", attrs: attributes! { class="w-[200px] justify-between!" },
///             combobox_value(placeholder: "Select framework...")
///             icon(data: iconify_icon!("lucide:chevrons-up-down"))
///         )
///         // `!` overrides the popover's `w-72`, `border-2` and `p-4`.
///         popover_content(id: "frameworks", attrs: attributes! { class="w-[anchor-size(width)]! border-0! p-0!" },
///             command(
///                 command_input(attrs: attributes! { placeholder="Search framework..." autofocus="" })
///                 command_list(
///                     command_empty("No framework found.")
///                     command_group(
///                         combobox_item(value: "next.js", "Next.js" combobox_item_indicator(attrs: attributes! { class="ml-auto" }))
///                         combobox_item(value: "remix", "Remix" combobox_item_indicator(attrs: attributes! { class="ml-auto" }))
///                     )
///                 )
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn combobox(
    /// The `name` the hidden input submits the value under.
    #[into]
    #[default]
    name: Option<&str>,
    /// The value the combobox starts with.
    #[into]
    #[default]
    value: Option<&str>,
    /// Whether several items can be checked at once.
    #[default]
    multiple: bool,
    #[default] mut attrs: Attributes,
    /// The trigger (or input) and the popover content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="combobox"
            data-multiple=(multiple)
            class=(class!("contents", attrs.remove("class")))
            (attrs)
        >
            <input type="hidden" name=(name) value=(value.unwrap_or_default())>
            (child)
        </div>
    })
}

/// The button that opens a combobox's popover: a `NoShadow` [`button`] look
/// with `role="combobox"`.
///
/// `target` is the `id` of the popover content. Put a [`combobox_value`] and
/// an icon inside. `attrs` are forwarded to the `<button>`, with extra classes
/// appended; use the `!` suffix (`px-2!`) to override the button's own.
///
/// [`button`]: super::button::button
#[component]
pub async fn combobox_trigger(
    /// The `id` of the popover content to open.
    target: &str,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        popover_trigger(
            target: target,
            attrs: attributes! {
                data-slot="combobox-trigger"
                role="combobox"
                class=(class!(
                    button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                    attrs.remove("class"),
                ))
                (attrs)
            },
            (child)
        )
    })
}

/// The choice shown in a [`combobox_trigger`], or `placeholder` while none is
/// checked.
///
/// Pass the initially checked item's content as children. After a choice, it
/// holds a copy of the chosen item's content, without its
/// [`combobox_item_indicator`] (or, in a `multiple` combobox, the chosen
/// labels). Content in an item marked
/// `hidden in-data-[slot=combobox-value]:inline` shows only here.
#[component]
pub async fn combobox_value(
    /// The text shown while nothing is checked.
    placeholder: &str,
    #[default] mut attrs: Attributes,
    /// The content of the initially checked item, if any.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="combobox-value"
            data-placeholder=(placeholder)
            class=(class!("empty:before:content-[attr(data-placeholder)]", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The autocomplete variant's input, anchored to the popover content whose
/// `id` is `target`, with a chevron button that opens it.
///
/// Typing opens the popover and filters the command list inside it; ArrowUp,
/// ArrowDown and Enter work from the input. Leave the command input out of
/// that command: this input is the search box. Choosing an item writes its
/// label into the input. `attrs` are forwarded to the [`input`]
/// (`placeholder`, `id`, ...), with extra classes appended.
///
/// ```ignore
/// view! {
///     combobox(name: "fruit",
///         combobox_input(target: "fruits", attrs: attributes! { placeholder="Pick a fruit" })
///         popover_content(id: "fruits", attrs: attributes! { class="w-[anchor-size(width)]! border-0! p-0!" },
///             command(
///                 command_list(
///                     command_empty("No fruit found.")
///                     command_group(
///                         combobox_item(value: "apple", "Apple")
///                         combobox_item(value: "banana", "Banana")
///                     )
///                 )
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn combobox_input(
    /// The `id` of the popover content that lists the choices.
    target: &str,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="combobox-input-group" class="relative w-full" style=(popover_anchor(target))>
            input(attrs: attributes! {
                data-slot="combobox-input"
                type="text"
                role="combobox"
                aria-controls=(target)
                aria-autocomplete="list"
                autocomplete="off"
                oninput=(AUTOCOMPLETE_SCRIPT)
                onkeydown=(AUTOCOMPLETE_SCRIPT)
                class=(class!("pr-10", attrs.remove("class")))
                (attrs)
            })
            popover_trigger(
                target: target,
                anchor: false,
                attrs: attributes! {
                    tabindex="-1"
                    aria-label="Show suggestions"
                    onclick="this.previousElementSibling.focus()"
                    class="absolute inset-y-0 right-0 flex items-center px-3 text-foreground \
                        [&_svg]:size-4 [&_svg]:shrink-0"
                },
                icon(data: iconify_icon!("lucide:chevrons-up-down"))
            )
        </div>
    })
}

/// A choice in a combobox: a [`command_item`] with a `value`.
///
/// Choosing it checks it (`data-checked="true"`), which a
/// [`combobox_item_indicator`] shows. `label` is the text written to a
/// [`combobox_input`] or joined in a `multiple` trigger; it defaults to the
/// item's text. Mark the item matching the combobox's initial `value` as
/// `checked`. `attrs` are forwarded to the command item; its `onclick` is
/// taken.
#[component]
pub async fn combobox_item(
    /// The value written to the hidden input.
    value: &str,
    /// The text shown for the item once chosen, if not its own text.
    #[into]
    #[default]
    label: Option<&str>,
    /// Whether the item starts checked.
    #[default]
    checked: bool,
    /// Whether the item can't be chosen.
    #[default]
    disabled: bool,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        command_item(
            value: value,
            disabled: disabled,
            attrs: attributes! {
                data-label=(label)
                data-checked=(if checked { "true" } else { "false" })
                onclick=(SELECT_SCRIPT)
                (attrs)
            },
            (child)
        )
    })
}

/// A check mark that shows while its [`combobox_item`] is checked. It is left
/// out of the copy shown in the trigger. `attrs` are forwarded to the `<svg>`,
/// with extra classes appended (`ml-auto` puts it at the end).
#[component]
pub async fn combobox_item_indicator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        icon(
            data: iconify_icon!("lucide:check"),
            attrs: attributes! {
                data-slot="combobox-item-indicator"
                class=(class!("opacity-0 in-data-[checked=true]:opacity-100", attrs.remove("class")))
                (attrs)
            }
        )
    })
}
