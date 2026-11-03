// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// A `pattern` that accepts digits only.
pub const REGEXP_ONLY_DIGITS: &str = "^\\d+$";

/// A `pattern` that accepts letters only.
#[allow(dead_code)]
pub const REGEXP_ONLY_CHARS: &str = "^[a-zA-Z]+$";

/// A `pattern` that accepts letters and digits.
pub const REGEXP_ONLY_DIGITS_AND_CHARS: &str = "^[a-zA-Z0-9]+$";

/// Classes for the slots' row. The slots ignore the pointer so every click
/// reaches the input laid over them.
const CONTAINER: StaticClass =
    class!("pointer-events-none relative flex items-center gap-2 has-disabled:opacity-50");

/// Classes for the real `<input>`: stretched invisibly over the slots, with
/// no visible text, caret or selection of its own.
const INPUT: StaticClass = class!(
    "pointer-events-auto absolute inset-0 size-full cursor-text appearance-none border-0 \
     bg-transparent font-mono leading-none tracking-[-0.5em] text-transparent \
     caret-transparent shadow-none outline-none selection:bg-transparent \
     disabled:cursor-not-allowed",
);

/// Keeps the slots in step with the input, on `input`, focus and key
/// events bubbling up from it.
///
/// It drops characters the `pattern` rejects (sending a fresh `input` event
/// so the caller's handler sees the cleaned value), writes each character
/// into the container's `--input-otp-char-N` property for slot `N` to show,
/// and marks the active slot. Editing always happens at the end of the code.
const SYNC: &str = "const t = event.target; if (t.tagName != 'INPUT') return; \
     const p = t.getAttribute('pattern'), m = t.maxLength; \
     if (event.type == 'input') { \
     const v = [...t.value].filter(c => !p || new RegExp(p).test(c)).join('').slice(0, m); \
     if (v != t.value) { t.value = v; t.dispatchEvent(new Event('input', { bubbles: true })); \
     return; } } \
     const n = t.value.length, f = event.type != 'focusout' && document.activeElement == t; \
     if (f) t.setSelectionRange(n, n); \
     for (let i = 0; i < m; i++) \
     this.style.setProperty('--input-otp-char-' + i, i < n ? JSON.stringify(t.value[i]) : ''); \
     this.querySelectorAll('[data-slot=input-otp-slot]').forEach(s => { \
     const a = f && +s.dataset.index == Math.min(n, m - 1); \
     s.dataset.active = a; s.dataset.caret = a && n < m; });";

/// Writes `c` as a quoted CSS string, for a slot's `content`.
fn css_string(c: char) -> String {
    match c {
        '\'' | '\\' => format!("'\\{c}'"),
        _ => format!("'{c}'"),
    }
}

/// A neobrutalism one-time-code input: one real `<input>` behind a row of
/// character slots.
///
/// Pass [`input_otp_group`]s of [`input_otp_slot`]s (and
/// [`input_otp_separator`]s) as children, one slot per character with
/// `index` from 0 to `max_length - 1`. `value` is the initial code.
/// `pattern` is a regular expression each typed character must match, such
/// as [`REGEXP_ONLY_DIGITS`]; others are dropped. It also becomes the
/// input's native `pattern`. The keyboard is numeric unless `pattern`
/// allows letters or `attrs` gives an `inputmode`.
///
/// `attrs` go on the `<input>` (`id`, `name`, `disabled`, `@input` reading
/// `e.target.value`), with classes appended to its classes;
/// `container_class` is added to the slots' row.
///
/// Built without the `input-otp` library: a small inline script fills the
/// slots, so typing, deleting and pasting work at the end of the code but
/// the caret cannot be moved into the middle. When it drops a rejected
/// character the caller's `@input` runs twice, the second time with the
/// cleaned value. The blinking caret needs the theme's `animate-caret-blink`.
///
/// ```ignore
/// view! {
///     input_otp(max_length: 6, pattern: Some(REGEXP_ONLY_DIGITS),
///         input_otp_group(
///             input_otp_slot(index: 0)
///             input_otp_slot(index: 1)
///             input_otp_slot(index: 2)
///         )
///         input_otp_separator()
///         input_otp_group(
///             input_otp_slot(index: 3)
///             input_otp_slot(index: 4)
///             input_otp_slot(index: 5)
///         )
///     )
/// }
/// ```
#[component]
pub async fn input_otp(
    max_length: usize,
    #[default] value: &str,
    #[default] pattern: Option<&str>,
    #[default] container_class: Option<&str>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // The initial characters, rendered the way the script writes them, so
    // the slots show the code before any script runs.
    let chars: String = value
        .chars()
        .take(max_length)
        .enumerate()
        .map(|(i, c)| format!("--input-otp-char-{i}:{};", css_string(c)))
        .collect();
    let style = (!chars.is_empty()).then_some(chars);

    let numeric = pattern.is_none_or(|p| p == REGEXP_ONLY_DIGITS);
    let default_inputmode = !attrs.contains_key("inputmode");

    Ok(view! {
        <div
            data-input-otp-container=""
            class=(class!(CONTAINER, container_class))
            style=(style)
            oninput=(SYNC)
            // The other events reuse the compiled `oninput` handler, which
            // branches on `event.type`.
            onfocusin="this.oninput(event)"
            onfocusout="this.oninput(event)"
            onkeyup="this.oninput(event)"
            onclick="this.oninput(event)"
        >
            (child)
            <input
                data-slot="input-otp"
                autocomplete="one-time-code"
                if default_inputmode {
                    inputmode=(if numeric { "numeric" } else { "text" })
                }
                maxlength=(max_length)
                pattern=(pattern)
                value=(value)
                class=(class!(INPUT, attrs.remove("class")))
                (attrs)
            >
        </div>
    })
}

/// A run of [`input_otp_slot`]s drawn as one joined box.
#[component]
pub async fn input_otp_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="input-otp-group"
            class=(class!("flex items-center", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for a character box.
///
/// Octane's `isActive && "z-10 ring-1 ring-ring"` becomes the
/// `data-[active=true]:` forms, set by the input's script. The character is
/// the `::before` content, read from `--otp-char`.
const SLOT: StaticClass = class!(
    "relative flex size-10 items-center justify-center border-y-2 border-r-2 border-border \
     bg-secondary-background text-sm font-base text-foreground first:rounded-l-base \
     first:border-l-2 last:rounded-r-base transition-all aria-invalid:border-red-500 \
     aria-invalid:ring-red-500 data-[active=true]:z-10 data-[active=true]:ring-1 \
     data-[active=true]:ring-ring before:content-(--otp-char)",
);

/// One character box of an [`input_otp`], showing character `index`.
///
/// `attrs` are forwarded to the `<div>`, with extra classes appended. Pass
/// `aria-invalid="true"` for the red error border.
#[component]
pub async fn input_otp_slot(
    index: usize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="input-otp-slot"
            data-index=(index)
            data-active="false"
            class=(class!(SLOT, attrs.remove("class")))
            style=(format!("--otp-char:var(--input-otp-char-{index})"))
            (attrs)
        >
            <div
                class="pointer-events-none absolute inset-0 hidden items-center justify-center \
                    in-data-[caret=true]:flex"
            >
                <div class="h-4 w-px animate-caret-blink bg-current duration-1000"></div>
            </div>
        </div>
    })
}

/// A dot between [`input_otp_group`]s.
#[component]
pub async fn input_otp_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="input-otp-separator"
            role="separator"
            class=(class!(attrs.remove("class")))
            (attrs)
        >
            icon(data: iconify_icon!("lucide:dot"), attrs: attributes! { class="size-4" })
        </div>
    })
}
