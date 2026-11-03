//! A multi-step form that asks one question at a time: single choice,
//! multiple choice or free text, with a progress label, validation and
//! keyboard shortcuts.
//!
//! The markup is a native `<form>`: each [`questionnaire_item`] is a
//! `<fieldset>`, choices are real radio buttons or checkboxes, and answers
//! submit with the form, so read them from `FormData` (or let the browser
//! post them). Choice styling reads the native inputs (`has-checked:`).
//!
//! The steps are driven by a small script that [`questionnaire`] renders
//! once per page (`window.neoQuestionnaire`), wired up through inline
//! handlers. It shows one item at a time, fills in the progress label,
//! shows and hides the navigation buttons, validates the current item before
//! moving on (and every item before submitting), and handles the keyboard
//! like the shadcn/ui primitive:
//!
//! - Enter on a filled answer, or Ctrl/Cmd+Enter anywhere, confirms the item
//!   (moves on, or submits on the last one).
//! - Arrow Left / Right move between items, Arrow Up / Down between answers.
//! - With `shortcuts`, a letter or number key picks the matching choice.
//!
//! Each step change fires a bubbling `itemchange` event on the new item's
//! `<fieldset>`, whose `name` is the item's name: handle it with
//! `@itemchange=$(|e: Event| step.set(e.target.name))` on the questionnaire
//! to drive other UI (Octane's `onItemChange`).
//!
//! Not matched from Octane: there is no `items` prop (the steps are the
//! rendered items, in order) and no controlled `item` / `defaultItem`; the
//! questionnaire always starts at its first item. Skipping clears the
//! item's answers. Native constraint validation (`noValidate={false}`) is not
//! supported. The script runs on a full page load; markup inserted later
//! needs `neoQuestionnaire.init(form)`.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    runtime::Expr,
    view::{Attributes, Child, StaticClass, Unescaped, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// The keys that pick a [`questionnaire_choice`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum QuestionnaireShortcuts {
    /// A to Z, in the order the enabled choices appear.
    Letters,
    /// 1 to 9, in the order the enabled choices appear.
    Numbers,
}

impl QuestionnaireShortcuts {
    /// The value of the form's `data-shortcuts` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Letters => "letters",
            Self::Numbers => "numbers",
        }
    }
}

/// The behavior shared by every questionnaire on the page, defined once.
const SCRIPT: &str = r#"
window.neoQuestionnaire ??= (() => {
  const own = (f, i) => i.closest('form') === f;
  const allItems = (f) => [...f.querySelectorAll('[data-slot=questionnaire-item]')].filter((i) => own(f, i));
  const items = (f) => allItems(f).filter((i) => !i.disabled);
  const answers = (i) => [...i.querySelectorAll('[data-slot=questionnaire-choice-input], [data-slot=questionnaire-input]')].filter((a) => !a.disabled);
  const filled = (a) => (a.type === 'radio' || a.type === 'checkbox' ? a.checked : a.value.trim() !== '');
  const isText = (el) => el.matches('textarea, select, [contenteditable], input:not([type=button], [type=checkbox], [type=radio], [type=reset], [type=submit])');
  const active = (f) => items(f).find((i) => i.hasAttribute('data-active')) ?? items(f)[0];

  // answered, skipped or unanswered, kept in `data-status`.
  const status = (i) => {
    const s = answers(i).some(filled) ? 'answered' : i.dataset.status === 'skipped' ? 'skipped' : 'unanswered';
    i.dataset.status = s;
    return s;
  };
  // Valid when answered (and not flagged with `invalid`), or skipped and optional.
  const valid = (i) => {
    const s = status(i);
    return (s === 'skipped' && i.dataset.required !== 'true') || (s === 'answered' && !i.hasAttribute('data-invalid'));
  };
  const describe = (i) => {
    const shown = i.hasAttribute('data-error') || i.hasAttribute('data-invalid');
    const ids = [...i.querySelectorAll('[data-slot=questionnaire-description]' + (shown ? ', [data-slot=questionnaire-error]' : ''))].map((el) => el.id);
    if (ids.length) i.setAttribute('aria-describedby', ids.join(' '));
    else i.removeAttribute('aria-describedby');
  };
  // Validates an item and shows or clears its error.
  const check = (i) => {
    const ok = valid(i);
    i.setAttribute('data-attempted', '');
    i.toggleAttribute('data-error', !ok);
    if (ok) i.removeAttribute('aria-invalid');
    else i.setAttribute('aria-invalid', 'true');
    describe(i);
    return ok;
  };
  const focusInvalid = (i) => (i.querySelector('input:checked:not(:disabled)') ?? answers(i)[0] ?? i).focus();

  function show(f, item, focus, notify = true) {
    if (!item) return;
    const list = items(f);
    const index = list.indexOf(item);
    const total = list.length;
    const changed = !item.hasAttribute('data-active');
    for (const i of allItems(f)) {
      const on = i === item;
      i.toggleAttribute('data-active', on);
      i.hidden = !on;
      i.inert = !on;
    }
    const button = (slot, visible) => {
      for (const b of f.querySelectorAll(`[data-slot=questionnaire-${slot}]`)) {
        b.hidden = !visible;
        b.inert = !visible;
      }
    };
    button('previous', total > 1 && index > 0);
    button('next', total > 1 && index < total - 1);
    button('submit', index === total - 1);
    button('skip', item.dataset.required !== 'true');
    const label = `Question ${index + 1} of ${total}`;
    for (const p of f.querySelectorAll('[data-slot=questionnaire-progress]')) {
      if (p.dataset.auto !== undefined) p.textContent = label;
      p.setAttribute('aria-valuemin', 1);
      p.setAttribute('aria-valuemax', total);
      p.setAttribute('aria-valuenow', index + 1);
      p.setAttribute('aria-valuetext', label);
    }
    if (focus) item.focus();
    if (changed && notify) item.dispatchEvent(new Event('itemchange', { bubbles: true }));
  }

  let uid = 0;
  const api = {
    init(f) {
      if (f.dataset.ready !== undefined) return;
      f.dataset.ready = '';
      const keys = { letters: 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', numbers: '123456789' }[f.dataset.shortcuts] ?? '';
      for (const i of allItems(f)) {
        const name = i.getAttribute('name');
        const type = i.hasAttribute('data-multiple') ? 'checkbox' : 'radio';
        let n = 0;
        for (const c of i.querySelectorAll('[data-slot=questionnaire-choice]')) {
          const input = c.querySelector('[data-slot=questionnaire-choice-input]');
          input.type = type;
          input.name = name;
          c.dataset.type = type;
          const key = input.disabled ? undefined : keys[n++];
          if (key) {
            c.dataset.shortcut = key;
            c.querySelector('[data-slot=questionnaire-choice-shortcut]').textContent = key;
            input.setAttribute('aria-keyshortcuts', key);
          }
        }
        for (const input of i.querySelectorAll('[data-slot=questionnaire-input]')) input.name = name;
        for (const el of i.querySelectorAll('[data-slot=questionnaire-description], [data-slot=questionnaire-error]')) el.id ||= `questionnaire-${++uid}`;
        status(i);
        describe(i);
      }
      for (const p of f.querySelectorAll('[data-slot=questionnaire-progress]')) if (!p.textContent.trim()) p.dataset.auto = '';
      show(f, active(f), false, false);
    },
    previous(f) {
      api.init(f);
      const list = items(f);
      const k = list.indexOf(active(f));
      if (k > 0) show(f, list[k - 1], true);
    },
    next(f) {
      api.init(f);
      const list = items(f);
      const i = active(f);
      const k = list.indexOf(i);
      if (k >= list.length - 1) return;
      if (!check(i)) return focusInvalid(i);
      show(f, list[k + 1], true);
    },
    skip(f) {
      api.init(f);
      const list = items(f);
      const i = active(f);
      if (!i || i.dataset.required === 'true') return;
      for (const a of answers(i)) {
        if (a.type === 'radio' || a.type === 'checkbox') a.checked = false;
        else a.value = '';
      }
      i.dataset.status = 'skipped';
      check(i);
      const k = list.indexOf(i);
      if (k < list.length - 1) show(f, list[k + 1], true);
      else api.submit(f);
    },
    // Validates every item up to the first invalid one, and shows that one.
    validate(f) {
      api.init(f);
      const bad = items(f).find((i) => !check(i));
      if (bad) {
        show(f, bad, false);
        focusInvalid(bad);
      }
      return !bad;
    },
    submit(f) {
      if (api.validate(f)) f.requestSubmit();
    },
    confirm(f) {
      const list = items(f);
      const i = active(f);
      if (!check(i)) return focusInvalid(i);
      const k = list.indexOf(i);
      if (k === list.length - 1) api.submit(f);
      else show(f, list[k + 1], true);
    },
    input(f, e) {
      api.init(f);
      const i = e.target.closest('[data-slot=questionnaire-item]');
      if (!i || !own(f, i)) return;
      i.dataset.status = 'unanswered';
      if (i.hasAttribute('data-attempted')) check(i);
      else status(i);
    },
    keydown(f, e) {
      if (e.defaultPrevented || e.isComposing || e.keyCode === 229 || !(e.target instanceof Element)) return;
      api.init(f);
      const i = active(f);
      if (!i) return;
      if (e.key === 'Enter' && (e.metaKey || e.ctrlKey) && !e.altKey && !e.shiftKey) {
        e.preventDefault();
        if (!e.repeat) api.confirm(f);
        return;
      }
      if (e.metaKey || e.ctrlKey || e.altKey) return;
      const list = answers(i);
      const k = list.indexOf(e.target);
      if (e.key === 'ArrowUp' || e.key === 'ArrowDown') {
        // Radios move within their group natively.
        if (e.target === i || (k >= 0 && e.target.type === 'checkbox')) {
          const down = e.key === 'ArrowDown';
          const to = k < 0 ? (list.find(filled) ?? list[down ? 0 : list.length - 1]) : list[(k + (down ? 1 : -1) + list.length) % list.length];
          if (to && to !== e.target) {
            e.preventDefault();
            to.focus();
            if (to.type === 'radio') to.click();
          }
          return;
        }
      }
      if ((e.key === 'ArrowLeft' || e.key === 'ArrowRight') && !isText(e.target) && e.target.type !== 'radio') {
        e.preventDefault();
        if (e.repeat) return;
        if (e.key === 'ArrowLeft') api.previous(f);
        else if (status(i) !== 'unanswered') api.next(f);
        return;
      }
      if (e.key === 'Enter') {
        if (k < 0) return;
        e.preventDefault();
        if (!e.repeat && filled(e.target)) api.confirm(f);
        return;
      }
      if (!f.dataset.shortcuts || isText(e.target) || e.key.length !== 1) return;
      const key = e.key.toUpperCase();
      const choice = [...i.querySelectorAll('[data-slot=questionnaire-choice]')].find((c) => c.dataset.shortcut === key);
      if (!choice) return;
      e.preventDefault();
      if (e.repeat) return;
      const input = choice.querySelector('[data-slot=questionnaire-choice-input]');
      input.focus();
      input.click();
    },
    reset(f) {
      // The fields reset after the event, so look once they have.
      setTimeout(() => {
        for (const i of allItems(f)) {
          i.dataset.status = 'unanswered';
          for (const name of ['data-attempted', 'data-error', 'aria-invalid']) i.removeAttribute(name);
          status(i);
          describe(i);
        }
        show(f, items(f)[0], false);
      });
    },
  };
  return api;
})();
"#;

/// Starts the questionnaire the script is in.
const INIT: &str = "neoQuestionnaire.init(document.currentScript.parentElement);";

/// Classes for the form. Until the script has run, CSS shows only the first
/// item.
const ROOT: StaticClass = class!(
    "flex w-full min-w-0 flex-col gap-6 \
     [&:not([data-ready])>fieldset:not(:first-of-type)]:hidden",
);

/// A questionnaire: a `<form>` holding a [`questionnaire_progress`], its
/// [`questionnaire_item`]s and a [`questionnaire_actions`] row.
///
/// `shortcuts` turns on letter or number keys for the choices. `attrs` are
/// forwarded to the `<form>`, with extra classes appended; handle `@submit`
/// there and read the answers from the form (skipped and unanswered items
/// submit nothing). The form sets `onkeydown`, `oninput` and `onreset`
/// itself.
///
/// ```ignore
/// view! {
///     questionnaire(attrs: attributes! { method="post" action="/survey" },
///         questionnaire_progress()
///         questionnaire_item(name: "plan", required: true,
///             questionnaire_title("Choose a plan")
///             questionnaire_choices(
///                 questionnaire_choice(value: "free", "Free")
///                 questionnaire_choice(value: "pro", "Pro")
///             )
///             questionnaire_error()
///         )
///         questionnaire_item(name: "notes",
///             questionnaire_title("Anything else?")
///             questionnaire_input(attrs: attributes! { placeholder="Type your answer..." })
///         )
///         questionnaire_actions(
///             questionnaire_previous()
///             questionnaire_skip()
///             questionnaire_next()
///             questionnaire_submit()
///         )
///     )
/// }
/// ```
#[component]
pub async fn questionnaire(
    /// Keys that pick a choice, if any.
    #[into]
    #[default]
    shortcuts: Option<QuestionnaireShortcuts>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <form
            data-slot="questionnaire"
            novalidate=""
            data-shortcuts=(shortcuts.map(QuestionnaireShortcuts::as_str))
            onkeydown="neoQuestionnaire.keydown(this, event)"
            oninput="neoQuestionnaire.input(this, event)"
            onreset="neoQuestionnaire.reset(this)"
            class=(class!(ROOT, attrs.remove("class")))
            (attrs)
        >
            (child)
            // Trusted constants, so they are written out unescaped.
            <script>(Unescaped::new_unchecked(SCRIPT)) (Unescaped::new_unchecked(INIT))</script>
        </form>
    })
}

/// "Question 2 of 3", kept up to date as the questionnaire moves. Pass
/// children to show your own text instead; the progress values are still
/// set on its `aria-` attributes.
#[component]
pub async fn questionnaire_progress(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="questionnaire-progress"
            role="progressbar"
            aria-label="Questionnaire progress"
            aria-live="polite"
            class=(class!(
                "min-h-[1lh] w-fit min-w-[14ch] text-xs font-base text-foreground tabular-nums",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One question, rendered as a `<fieldset>`: a [`questionnaire_title`], an
/// optional [`questionnaire_description`], [`questionnaire_choices`] or a
/// [`questionnaire_input`], and a [`questionnaire_error`].
///
/// `name` is the form field name of its answers. A `required` item must be
/// answered; an optional one can be skipped. `multiple` makes its choices
/// checkboxes. `disabled` leaves it out of the questionnaire. `invalid`
/// marks the answer as wrong (shows the error and blocks navigation); pass a
/// runtime expression to check it as the user types.
#[component]
pub async fn questionnaire_item(
    /// The form field name of the item's answers.
    name: &str,
    /// Whether the item must be answered.
    #[default]
    required: bool,
    /// Whether several choices can be picked.
    #[default]
    multiple: bool,
    /// Leaves the item out of the questionnaire.
    #[default]
    disabled: bool,
    /// Marks the current answer as invalid.
    #[into]
    #[default(Expr::from(false))]
    invalid: Expr<bool>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <fieldset
            data-slot="questionnaire-item"
            name=(name)
            data-required=(if required { "true" } else { "false" })
            data-multiple=(multiple)
            data-status="unanswered"
            disabled=(disabled)
            tabindex="-1"
            :data-invalid=(invalid)
            class=(class!(
                "group/questionnaire-item flex min-w-0 flex-col gap-5 border-0 p-0 outline-none",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </fieldset>
    })
}

/// The question, rendered as the item's `<legend>`.
#[component]
pub async fn questionnaire_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <legend
            data-slot="questionnaire-title"
            class=(class!(
                "text-base leading-snug font-heading text-foreground text-pretty \
                 [&:not(:has(~[data-slot=questionnaire-description]))]:mb-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </legend>
    })
}

/// Help text under the question. The item is described by it.
#[component]
pub async fn questionnaire_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="questionnaire-description"
            class=(class!("text-sm font-base text-pretty text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}

/// The list of [`questionnaire_choice`]s of an item.
#[component]
pub async fn questionnaire_choices(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="questionnaire-choices"
            class=(class!("group/questionnaire-choices grid min-w-0 gap-3", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for a choice card. Base UI's `data-checked:` and `data-disabled:`
/// become `has-checked:` and `has-disabled:`, read from the native input.
const CHOICE: StaticClass = class!(
    "group/questionnaire-choice relative flex min-h-11 cursor-pointer items-start gap-3 \
     rounded-base border-2 border-border bg-secondary-background px-4 py-3 text-start text-sm \
     font-base text-foreground transition-colors outline-none select-none \
     has-[>input:focus-visible]:ring-2 has-[>input:focus-visible]:ring-black \
     has-[>input:focus-visible]:ring-offset-2 has-checked:bg-background \
     has-disabled:pointer-events-none has-disabled:cursor-not-allowed has-disabled:opacity-50",
);

/// Classes for the box (square for checkboxes, round for radios).
const INDICATOR: StaticClass = class!(
    "pointer-events-none relative flex size-4 shrink-0 translate-y-[--spacing(0.45)] items-center \
     justify-center rounded-base border-2 border-border bg-secondary-background \
     text-main-foreground \
     group-has-data-[slot=questionnaire-choice-description]/questionnaire-choice:translate-y-0.5 \
     group-data-[type=radio]/questionnaire-choice:rounded-full \
     group-has-checked/questionnaire-choice:bg-main \
     group-has-checked/questionnaire-choice:group-data-[type=radio]/questionnaire-choice:bg-transparent",
);

/// Classes for the round mark of a picked radio.
const DOT: StaticClass = class!(
    "hidden size-2 rounded-full bg-current \
     group-data-[type=checkbox]/questionnaire-choice:hidden \
     group-has-checked/questionnaire-choice:block",
);

/// Classes for the check mark of a picked checkbox.
const CHECK: StaticClass = class!(
    "hidden size-3.5 group-data-[type=radio]/questionnaire-choice:hidden \
     group-has-checked/questionnaire-choice:block",
);

/// Classes for the shortcut key badge, shown once a key is assigned.
const SHORTCUT: StaticClass = class!(
    "pointer-events-none ms-auto hidden size-5 shrink-0 items-center justify-center rounded-full \
     border-2 border-border bg-secondary-background text-xs leading-none font-base text-foreground \
     group-has-checked/questionnaire-choice:bg-background \
     group-data-[shortcut]/questionnaire-choice:inline-flex",
);

/// One answer of an item: a card with a native radio button (or checkbox,
/// in a `multiple` item) laid over it, a mark, the label and its shortcut
/// key.
///
/// `value` is the submitted value. `checked` picks it initially (Octane's
/// `defaultChecked`) and `disabled` greys it out. The script gives the input
/// its item's `name` and type. Classes and other attributes in `attrs` go on
/// the `<label>`. Add a [`questionnaire_choice_description`] after the label
/// text for a second line.
#[component]
pub async fn questionnaire_choice(
    /// The value submitted when the choice is picked.
    value: &str,
    /// Picks the choice initially.
    #[default]
    checked: bool,
    /// Makes the choice unavailable.
    #[default]
    disabled: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <label
            data-slot="questionnaire-choice"
            data-type="radio"
            class=(class!(CHOICE, attrs.remove("class")))
            (attrs)
        >
            <input
                data-slot="questionnaire-choice-input"
                type="radio"
                value=(value)
                checked=(checked)
                disabled=(disabled)
                class="absolute inset-0 z-10 size-full cursor-pointer opacity-0"
            >
            <span aria-hidden="true" data-slot="questionnaire-choice-indicator" class=(INDICATOR)>
                <span data-slot="questionnaire-choice-indicator-dot" class=(DOT)></span>
                icon(
                    data: iconify_icon!("lucide:check"),
                    attrs: attributes! {
                        data-slot="questionnaire-choice-indicator-check"
                        class=(CHECK)
                    }
                )
            </span>
            <span
                data-slot="questionnaire-choice-label"
                class="flex min-w-0 flex-1 flex-col gap-0.5 leading-snug"
            >
                (child)
            </span>
            <span data-slot="questionnaire-choice-shortcut" aria-hidden="true" class=(SHORTCUT)></span>
        </label>
    })
}

/// A second line of text inside a [`questionnaire_choice`].
#[component]
pub async fn questionnaire_choice_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="questionnaire-choice-description"
            class=(class!("text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </span>
    })
}

/// Classes for the free-text input.
const INPUT: StaticClass = class!(
    "flex h-10 min-h-10 w-full min-w-0 rounded-base border-2 border-border bg-secondary-background \
     px-3 py-2 text-base font-base text-foreground focus-visible:outline-hidden \
     focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2 \
     disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-50 sm:min-h-0 \
     md:text-sm selection:bg-main selection:text-main-foreground placeholder:text-foreground/50",
);

/// A free-text answer: a native `<input>` that takes its item's `name`.
///
/// Every attribute in `attrs` goes on the `<input>`: `type` (text by
/// default), `placeholder`, `aria-label`, `autocomplete`, `@input`. Name it
/// with `aria-label`, since the item's title is the fieldset's legend.
#[component]
pub async fn questionnaire_input(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="questionnaire-input-wrapper"
            class="group/questionnaire-input relative w-full min-w-0"
        >
            <input
                data-slot="questionnaire-input"
                class=(class!(INPUT, attrs.remove("class")))
                (attrs)
            >
        </div>
    })
}

/// The message shown when an item's answer is missing or invalid: after a
/// failed Next or Submit, and while its item is marked `invalid`.
///
/// Without children it says "Choose an answer to continue." on a required
/// item and "Choose an answer or skip this question." on an optional one.
#[component]
pub async fn questionnaire_error(
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="questionnaire-error"
            role="alert"
            class=(class!(
                "mt-2 text-sm font-base text-red-500",
                "hidden group-data-error/questionnaire-item:block \
                 group-data-invalid/questionnaire-item:block",
                attrs.remove("class"),
            ))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => {
                    <span class="group-data-[required=false]/questionnaire-item:hidden">
                        "Choose an answer to continue."
                    </span>
                    <span class="group-data-[required=true]/questionnaire-item:hidden">
                        "Choose an answer or skip this question."
                    </span>
                },
            }
        </p>
    })
}

/// The row of navigation buttons at the end of a [`questionnaire`]:
/// Previous on the left, then Skip and Next (or Submit) on the right.
#[component]
pub async fn questionnaire_actions(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="questionnaire-actions"
            class=(class!(
                "grid min-h-11 w-full grid-cols-[minmax(0,1fr)_auto_auto] items-center gap-2 sm:min-h-8",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Goes back one item. Hidden on the first item. `variant` defaults to
/// `Neutral` and `size` to `Sm`; children replace the "Previous" label.
#[component]
pub async fn questionnaire_previous(
    #[default(ButtonVariant::Neutral)] variant: ButtonVariant,
    #[default(ButtonSize::Sm)] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="questionnaire-previous"
            hidden=""
            onclick="neoQuestionnaire.previous(this.form)"
            class=(class!(
                button_variants(variant, size),
                "col-start-1 row-start-1 min-h-11 justify-self-start sm:min-h-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => "Previous",
            }
        </button>
    })
}

/// Skips an optional item, clearing its answer. Hidden on required items.
/// `variant` defaults to `Neutral` and `size` to `Sm`; children replace the
/// "Skip" label.
#[component]
pub async fn questionnaire_skip(
    #[default(ButtonVariant::Neutral)] variant: ButtonVariant,
    #[default(ButtonSize::Sm)] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="questionnaire-skip"
            onclick="neoQuestionnaire.skip(this.form)"
            class=(class!(
                button_variants(variant, size),
                "col-start-2 row-start-1 min-h-11 justify-self-end sm:min-h-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => "Skip",
            }
        </button>
    })
}

/// Checks the current item and moves to the next one. Hidden on the last
/// item. `variant` defaults to `Default` and `size` to `Sm`; children replace
/// the "Next" label.
#[component]
pub async fn questionnaire_next(
    #[default] variant: ButtonVariant,
    #[default(ButtonSize::Sm)] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="questionnaire-next"
            aria-keyshortcuts="Enter"
            onclick="neoQuestionnaire.next(this.form)"
            class=(class!(
                button_variants(variant, size),
                "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => "Next",
            }
        </button>
    })
}

/// Submits the form once every item is valid; otherwise it shows the first
/// invalid one. Shown on the last item only. `variant` defaults to `Default`
/// and `size` to `Sm`; children replace the "Submit" label.
#[component]
pub async fn questionnaire_submit(
    #[default] variant: ButtonVariant,
    #[default(ButtonSize::Sm)] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[into]
    #[default]
    child: Option<Child<'_>>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="submit"
            data-slot="questionnaire-submit"
            aria-keyshortcuts="Enter"
            hidden=""
            onclick="if (!neoQuestionnaire.validate(this.form)) event.preventDefault()"
            class=(class!(
                button_variants(variant, size),
                "col-start-3 row-start-1 min-h-11 justify-self-end sm:min-h-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            match child {
                Some(child) => (child),
                None => "Submit",
            }
        </button>
    })
}
