//! Toasts: short messages that stack in a corner and dismiss themselves, in
//! place of sonner.
//!
//! Render one [`toaster`] per page, usually in the root layout. It is the
//! fixed region the toasts stack in (Base UI's provider, portal and viewport
//! in one), and it defines a global JavaScript function that shows toasts
//! from anywhere in the page, such as a button's `onclick`:
//!
//! ```js
//! toast({ title: 'Event created', description: 'Sunday, December 3 at 9:00 AM' })
//! toast({ variant: 'success', description: 'Saved.', action: { label: 'Undo', onClick: undo } })
//! toast('Copied.')                      // a string is the description
//! const id = toast({ variant: 'loading', description: 'Uploading…' })
//! toast.update(id, { variant: 'success', description: 'Uploaded.' })
//! toast.close(id)
//! toast.promise(save(), { loading: 'Saving…', success: 'Saved.', error: (e) => e.message })
//! ```
//!
//! The options are `title`, `description`, `variant` (`default`, `success`,
//! `info`, `warning`, `error` or `loading`, which sets the icon), `action`
//! (`{ label, onClick }`; clicking it runs `onClick` and then closes the toast
//! unless `onClick` calls `event.preventDefault()`), `priority: 'high'`
//! (announced as an alert) and `timeout` in milliseconds (`0` keeps it up).
//! Loading toasts stay up until updated.
//!
//! Toasts the server renders, such as a flash message after a form post, go
//! in the toaster as [`toast`] children; they slide in and dismiss
//! themselves the same way.
//!
//! The stack behaves like Base UI's: the newest toast in front with up to
//! `limit - 1` peeking behind it, all of them spread out while the pointer or
//! keyboard focus is in the toaster, which also pauses their timers. Escape
//! closes the focused toast. Swiping to dismiss is not supported.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, Unescaped, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};

/// Defines `window.toast` once per page; each toaster's copy then starts the
/// toasts the server rendered into it. It runs from a `<script>` at the end
/// of the [`toaster`].
const TOAST_SCRIPT: &str = r#"
window.toast ??= (() => {
  const state = new WeakMap(); // toast element -> { options, left, start, timer }
  let count = 0;

  // A string is shorthand for `{ description }`.
  const options = (o) => (typeof o === 'string' ? { description: o } : { ...o });
  // The toast with an id that `toast()` returned, or the toast holding an element.
  const find = (ref) =>
    typeof ref === 'string'
      ? document.querySelector(`[data-slot=toast][data-toast-id="${ref}"]`)
      : ref?.closest('[data-slot=toast]');

  // Base UI's stack: sets the CSS variables and data attributes the toast
  // classes read. The newest toast is index 0.
  const layout = (region) => {
    const toasts = [...region.querySelectorAll(':scope > [data-slot=toast]')].reverse();
    const expanded = region.hasAttribute('data-expanded');
    const limit = Number(region.dataset.limit);
    let index = 0;
    let offset = 0;
    let front;
    for (const t of toasts) {
      const content = t.querySelector('[data-slot=toast-content]');
      // The natural height, outside the stacked height the classes give it.
      const height = t.style.height;
      t.style.height = 'auto';
      const natural = t.offsetHeight;
      t.style.height = height;
      if (!t.hasAttribute('data-ending-style')) {
        front ??= natural;
        t.style.setProperty('--toast-index', index);
        t.toggleAttribute('data-limited', index >= limit);
        t.inert = index >= limit;
        content.toggleAttribute('data-behind', index > 0);
        index += 1;
      }
      t.style.setProperty('--toast-height', `${natural}px`);
      t.style.setProperty('--toast-offset-y', `${offset}px`);
      t.toggleAttribute('data-expanded', expanded);
      content.toggleAttribute('data-expanded', expanded);
      offset += natural;
    }
    region.style.setProperty('--toast-frontmost-height', `${front ?? 0}px`);
  };

  // Auto-dismiss timers, paused while the toaster is expanded.
  const pause = (t) => {
    const s = state.get(t);
    if (!s?.timer) return;
    clearTimeout(s.timer);
    s.timer = 0;
    s.left -= Date.now() - s.start;
  };
  const resume = (t) => {
    const s = state.get(t);
    if (!s || s.timer || !(s.left > 0)) return;
    s.start = Date.now();
    s.timer = setTimeout(() => close(t), s.left);
  };
  // (Re)starts a toast's timer. Loading toasts and a timeout of 0 stay up.
  const arm = (t) => {
    const s = state.get(t);
    const region = t.parentElement;
    pause(t);
    s.left = t.dataset.variant === 'loading'
      ? 0
      : Number(s.options.timeout ?? t.dataset.timeout ?? region.dataset.timeout);
    if (!region.hasAttribute('data-expanded')) resume(t);
  };

  // Writes the given options into a toast; options left out stay as they are.
  const fill = (t, o) => {
    if ('variant' in o) t.dataset.variant = o.variant ?? 'default';
    if ('priority' in o) {
      if (o.priority === 'high') t.setAttribute('role', 'alert');
      else t.removeAttribute('role');
    }
    for (const part of ['title', 'description']) {
      const e = t.querySelector(`[data-slot=toast-${part}]`);
      if (e && part in o) {
        e.textContent = o[part] ?? '';
        e.hidden = !o[part];
      }
    }
    const action = t.querySelector('[data-slot=toast-action]');
    if (action && 'action' in o) {
      action.textContent = o.action?.label ?? '';
      action.hidden = !o.action;
      action.onclick = (event) => {
        o.action?.onClick?.(event);
        if (!event.defaultPrevented) close(t);
      };
    }
  };

  // Slides new toasts in from their `data-starting-style` position and arms
  // their timers.
  const start = (region, toasts) => {
    for (const t of toasts) t.setAttribute('data-starting-style', '');
    layout(region);
    requestAnimationFrame(() => toasts.forEach((t) => t.removeAttribute('data-starting-style')));
    toasts.forEach(arm);
  };

  // Slides a toast out, then removes it once its 500ms transition is over.
  const close = (ref) => {
    const t = find(ref);
    if (!t || t.hasAttribute('data-ending-style')) return;
    const region = t.parentElement;
    pause(t);
    t.setAttribute('data-ending-style', '');
    t.inert = true;
    layout(region);
    setTimeout(() => {
      t.remove();
      layout(region);
    }, 500);
  };

  // Shows a toast in the first displayed toaster on the page and returns its
  // id, for `toast.update()` and `toast.close()`.
  const toast = (input) => {
    const toasters = [...document.querySelectorAll('[data-slot=toaster]')];
    const region = toasters.find((r) => r.checkVisibility?.() ?? true);
    if (!region) return;
    const t = region.querySelector(':scope > template').content.firstElementChild.cloneNode(true);
    t.dataset.toastId = String(++count);
    state.set(t, { options: options(input) });
    fill(t, state.get(t).options);
    region.append(t);
    start(region, [t]);
    return t.dataset.toastId;
  };

  toast.close = close;

  toast.update = (ref, input) => {
    const t = find(ref);
    const s = t && state.get(t);
    if (!s || t.hasAttribute('data-ending-style')) return;
    const o = options(input);
    s.options = { ...s.options, ...o };
    fill(t, o);
    arm(t);
    layout(t.parentElement);
  };

  // One toast for a promise: loading until it settles, then success or
  // error. Each option is a string, an options object, or a function of the
  // promise's result (or error).
  toast.promise = (promise, o) => {
    const id = toast({ ...options(o.loading), variant: 'loading' });
    const settle = (variant, option) => (result) =>
      toast.update(id, {
        ...options(typeof option === 'function' ? option(result) : option),
        variant,
      });
    promise.then(settle('success', o.success), settle('error', o.error));
    return promise;
  };

  // The toaster's pointer and focus handlers: expanded while the pointer or
  // focus is in it, with the timers paused.
  toast.expand = (region, event) => {
    if (event.type.startsWith('pointer')) {
      region.toggleAttribute('data-hovered', event.type === 'pointerenter');
    }
    const focus = event.type === 'focusout' ? event.relatedTarget : document.activeElement;
    const expanded = region.hasAttribute('data-hovered') || region.contains(focus);
    if (expanded === region.hasAttribute('data-expanded')) return;
    region.toggleAttribute('data-expanded', expanded);
    for (const t of region.querySelectorAll(':scope > [data-slot=toast]')) {
      if (expanded) pause(t);
      else resume(t);
    }
    layout(region);
  };

  // Starts the toasts the server rendered into a toaster.
  toast.mount = (region) => {
    const toasts = [...region.querySelectorAll(':scope > [data-slot=toast]')];
    for (const t of toasts) {
      t.dataset.toastId = String(++count);
      state.set(t, { options: {} });
    }
    start(region, toasts);
  };

  return toast;
})();
toast.mount(document.currentScript.parentElement);
"#;

/// Where a [`toaster`] stacks its toasts.
///
/// [`Default`] is `ToasterPosition::BottomRight`, used when no position is
/// given. On small screens every position spans the width, at the top or
/// bottom.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToasterPosition {
    /// The top-left corner.
    TopLeft,
    /// The middle of the top edge.
    TopCenter,
    /// The top-right corner.
    TopRight,
    /// The bottom-left corner.
    BottomLeft,
    /// The middle of the bottom edge.
    BottomCenter,
    /// The bottom-right corner.
    #[default]
    BottomRight,
}

impl ToasterPosition {
    /// The value of the toaster's `data-position` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            Self::TopRight => "top-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::BottomRight => "bottom-right",
        }
    }

    /// Classes that place the toaster. The toast classes stack upward from
    /// the bottom edge, so the top positions mirror the toaster vertically
    /// (`-scale-y-100`) and each toast's content mirrors itself back.
    fn classes(self) -> StaticClass {
        match self {
            Self::TopLeft => class!("top-4 -scale-y-100 sm:left-4 sm:right-auto sm:mx-0"),
            Self::TopCenter => class!("top-4 -scale-y-100"),
            Self::TopRight => class!("top-4 -scale-y-100 sm:right-4 sm:left-auto sm:mx-0"),
            Self::BottomLeft => class!("bottom-4 sm:left-4 sm:right-auto sm:mx-0"),
            Self::BottomCenter => class!("bottom-4"),
            Self::BottomRight => class!("bottom-4 sm:right-4 sm:left-auto sm:mx-0"),
        }
    }
}

/// Classes for the toaster region, apart from its position.
const VIEWPORT: StaticClass = class!(
    "pointer-events-none fixed inset-x-4 z-50 mx-auto w-auto max-w-sm outline-none sm:w-full",
);

/// The region toasts stack in, and the `toast()` function that shows them.
///
/// Render it once per page, usually at the end of the root layout's
/// `<body>`. Pass server-rendered [`toast`]s, such as flash messages, as
/// children. At most `limit` toasts show at once (the oldest wait behind),
/// and each one dismisses itself after `timeout` milliseconds unless it sets
/// its own. When a page has several toasters, `toast()` uses the first one
/// that is displayed. `attrs` are forwarded to the region `<div>`, with
/// extra classes appended.
///
/// ```ignore
/// view! {
///     <body>
///         (slot)
///         toaster(
///             if let Some(message) = flash {
///                 toast(variant: ToastVariant::Success, description: message)
///             }
///         )
///     </body>
/// }
/// ```
///
/// Then, anywhere in the page:
///
/// ```ignore
/// view! {
///     button(attrs: attributes! { onclick="toast({ title: 'Event created' })" }, "Show Toast")
/// }
/// ```
#[component]
pub async fn toaster(
    /// Where the toasts stack.
    #[default]
    position: ToasterPosition,
    /// How many toasts show at once.
    #[default(3)]
    limit: usize,
    /// How long a toast stays up, in milliseconds.
    #[default(5000)]
    timeout: u32,
    #[default] mut attrs: Attributes,
    /// Toasts rendered by the server.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="toaster"
            data-position=(position.as_str())
            data-limit=(limit)
            data-timeout=(timeout)
            role="region"
            aria-label="Notifications"
            aria-live="polite"
            aria-relevant="additions text"
            onpointerenter="toast.expand(this, event)"
            onpointerleave="toast.expand(this, event)"
            onfocusin="toast.expand(this, event)"
            onfocusout="toast.expand(this, event)"
            onkeydown="if (event.key === 'Escape') toast.close(event.target)"
            class=(class!(VIEWPORT, position.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
            // What `toast()` clones: a toast with every icon, the empty parts
            // hidden until it fills them.
            <template>toast_frame(template: true)</template>
            // A trusted constant, so it is written out unescaped.
            <script>(Unescaped::new_unchecked(TOAST_SCRIPT))</script>
        </div>
    })
}

/// The kind of message a [`toast`] carries, which picks its icon. The
/// JavaScript `variant` option takes the same names in lowercase.
///
/// [`Default`] is `ToastVariant::Default`, which shows no icon.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastVariant {
    /// A plain message, without an icon.
    #[default]
    Default,
    /// Something worked: a check in a circle.
    Success,
    /// Something to know: an "i" in a circle.
    Info,
    /// Something to watch out for: a triangle with "!".
    Warning,
    /// Something failed: an octagon with an "x".
    Error,
    /// Something is in progress: a spinner. Loading toasts stay up.
    Loading,
}

/// Every variant, in the order the template renders their icons.
const VARIANTS: [ToastVariant; 6] = [
    ToastVariant::Default,
    ToastVariant::Success,
    ToastVariant::Info,
    ToastVariant::Warning,
    ToastVariant::Error,
    ToastVariant::Loading,
];

impl ToastVariant {
    /// The value of the toast's `data-variant` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Success => "success",
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Loading => "loading",
        }
    }

    /// The variant's icon and its classes, if it has one. Each icon shows
    /// only while the toast has its variant, so the template can hold all of
    /// them and a toast can change variant.
    fn icon(self) -> Option<(IconData, &'static str)> {
        match self {
            Self::Default => None,
            Self::Success => Some((
                iconify_icon!("lucide:circle-check"),
                "hidden group-data-[variant=success]/toast:block",
            )),
            Self::Info => Some((
                iconify_icon!("lucide:info"),
                "hidden group-data-[variant=info]/toast:block",
            )),
            Self::Warning => Some((
                iconify_icon!("lucide:triangle-alert"),
                "hidden group-data-[variant=warning]/toast:block",
            )),
            Self::Error => Some((
                iconify_icon!("lucide:octagon-x"),
                "hidden text-black group-data-[variant=error]/toast:block",
            )),
            Self::Loading => Some((
                iconify_icon!("lucide:loader-circle"),
                "hidden animate-spin group-data-[variant=loading]/toast:block",
            )),
        }
    }
}

/// Classes for a toast: Base UI's stacking, sizing and enter/exit transforms,
/// driven by the CSS variables and data attributes the toaster's script sets.
const TOAST: StaticClass = class!(
    "group/toast pointer-events-auto absolute right-0 bottom-0 z-[calc(1000-var(--toast-index))] \
     w-full origin-bottom rounded-base border-2 border-border bg-background text-foreground \
     will-change-transform outline-none select-none focus-visible:ring-2 focus-visible:ring-black \
     focus-visible:ring-offset-2 \
     [--gap:0.75rem] [--height:var(--toast-frontmost-height,var(--toast-height))] \
     [--offset-y:calc(var(--toast-offset-y)*-1+calc(var(--toast-index)*var(--gap)*-1)+var(--toast-swipe-movement-y))] \
     [--peek:0.75rem] [--scale:calc(max(0,1-(var(--toast-index)*0.1)))] [--shrink:calc(1-var(--scale))] \
     h-(--height) [transform:translateX(var(--toast-swipe-movement-x))_translateY(calc(var(--toast-swipe-movement-y)-(var(--toast-index)*var(--peek))-(var(--shrink)*var(--height))))_scale(var(--scale))] \
     [transition:transform_500ms_cubic-bezier(0.22,1,0.36,1),opacity_500ms,height_150ms] \
     after:absolute after:top-full after:left-0 after:h-[calc(var(--gap)+1px)] after:w-full \
     after:content-[''] \
     data-expanded:h-(--toast-height) \
     data-expanded:[transform:translateX(var(--toast-swipe-movement-x))_translateY(var(--offset-y))] \
     data-limited:opacity-0 data-starting-style:[transform:translateY(150%)] \
     [&[data-ending-style]:not([data-limited]):not([data-swipe-direction])]:[transform:translateY(150%)] \
     data-ending-style:data-[swipe-direction=down]:[transform:translateY(calc(var(--toast-swipe-movement-y)+150%))] \
     data-ending-style:data-[swipe-direction=left]:[transform:translateX(calc(var(--toast-swipe-movement-x)-150%))_translateY(var(--offset-y))] \
     data-ending-style:data-[swipe-direction=right]:[transform:translateX(calc(var(--toast-swipe-movement-x)+150%))_translateY(var(--offset-y))] \
     data-ending-style:data-[swipe-direction=up]:[transform:translateY(calc(var(--toast-swipe-movement-y)-150%))] \
     data-expanded:data-ending-style:data-[swipe-direction=down]:[transform:translateY(calc(var(--toast-swipe-movement-y)+150%))] \
     data-expanded:data-ending-style:data-[swipe-direction=left]:[transform:translateX(calc(var(--toast-swipe-movement-x)-150%))_translateY(var(--offset-y))] \
     data-expanded:data-ending-style:data-[swipe-direction=right]:[transform:translateX(calc(var(--toast-swipe-movement-x)+150%))_translateY(var(--offset-y))] \
     data-expanded:data-ending-style:data-[swipe-direction=up]:[transform:translateY(calc(var(--toast-swipe-movement-y)-150%))]",
);

/// One toast rendered by the server, such as a flash message. Put it in the
/// [`toaster`].
///
/// `variant` picks the icon. Pass a [`toast_action`] as children to add an
/// action button. With `high_priority` it is announced as an alert. It
/// dismisses itself after the toaster's timeout, or `timeout` milliseconds
/// (`Some(0)` keeps it up). `attrs` are forwarded to the toast `<div>`, with
/// extra classes appended.
///
/// ```ignore
/// view! {
///     toaster(
///         toast(
///             variant: ToastVariant::Success,
///             title: "Profile saved",
///             description: "Your changes are live.",
///             toast_action(attrs: attributes! { onclick="history.back()" }, "Undo")
///         )
///     )
/// }
/// ```
#[component]
pub async fn toast(
    /// The kind of message, which picks the icon.
    #[default]
    variant: ToastVariant,
    /// The heading.
    #[into]
    #[default]
    title: Option<&str>,
    /// The message.
    #[into]
    #[default]
    description: Option<&str>,
    /// Whether to announce it as an alert.
    #[default]
    high_priority: bool,
    /// How long it stays up, in milliseconds, instead of the toaster's timeout.
    #[default]
    timeout: Option<u32>,
    #[default] attrs: Attributes,
    /// Action buttons, such as a [`toast_action`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        toast_frame(
            variant: variant,
            title: title,
            description: description,
            high_priority: high_priority,
            timeout: timeout,
            attrs: attrs,
            (child)
        )
    })
}

/// The markup of every toast: a server-rendered [`toast`], or, with
/// `template`, the one `toast()` clones, which holds every icon and a hidden
/// action button.
#[component]
async fn toast_frame(
    #[default] variant: ToastVariant,
    #[default] template: bool,
    #[default] title: Option<&str>,
    #[default] description: Option<&str>,
    #[default] high_priority: bool,
    #[default] timeout: Option<u32>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // The icons to render: all of them in the template, else this variant's.
    let icons = VARIANTS
        .into_iter()
        .filter(move |&v| template || v == variant)
        .filter_map(ToastVariant::icon);
    let has_icon = template || variant != ToastVariant::Default;

    Ok(view! {
        <div
            data-slot="toast"
            data-variant=(variant.as_str())
            data-timeout=(timeout)
            role=(high_priority.then_some("alert"))
            aria-atomic="true"
            tabindex="0"
            // No swipe gestures, so the swipe offsets the classes read are 0.
            style="--toast-swipe-movement-x: 0px; --toast-swipe-movement-y: 0px"
            class=(class!(TOAST, attrs.remove("class")))
            (attrs)
        >
            toast_content(
                if has_icon {
                    <span
                        data-slot="toast-icon"
                        class="shrink-0 group-data-[variant=default]/toast:hidden \
                            [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4"
                    >
                        for (glyph, classes) in icons {
                            icon(data: glyph, attrs: attributes! { class=(classes) })
                        }
                    </span>
                }
                <div class="flex min-w-0 flex-1 flex-col gap-1">
                    toast_title(attrs: attributes! { hidden=(title.is_none()) }, (title.unwrap_or_default()))
                    toast_description(
                        attrs: attributes! { hidden=(description.is_none()) },
                        (description.unwrap_or_default())
                    )
                </div>
                if template {
                    toast_action(attrs: attributes! { hidden="" })
                } else {
                    (child)
                }
                toast_close()
            )
        </div>
    })
}

/// The row inside a [`toast`]: icon, text, actions and close button. It
/// fades out behind the frontmost toast while the stack is collapsed.
#[component]
pub async fn toast_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // `in-data-[position^=top]:-scale-y-100` mirrors the content back in a
    // toaster at the top, which is mirrored itself.
    Ok(view! {
        <div
            data-slot="toast-content"
            class=(class!(
                "flex h-full items-center gap-3 overflow-hidden px-4 py-3 transition-opacity \
                 duration-250 ease-[cubic-bezier(0.22,1,0.36,1)] data-behind:opacity-0 \
                 data-expanded:opacity-100 in-data-[position^=top]:-scale-y-100",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`toast`], rendered as an `<h2>`.
#[component]
pub async fn toast_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="toast-title"
            class=(class!("text-sm font-heading", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// The message of a [`toast`], rendered as a `<p>`.
#[component]
pub async fn toast_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="toast-description"
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}

/// An action button for a server-rendered [`toast`], styled as a small
/// `NoShadow` button on the secondary background. Clicking it closes the
/// toast; an `onclick` in `attrs` replaces that, so end yours with
/// `toast.close(this)`. Other attributes (`form`, `formaction`, ...) are
/// forwarded to the `<button>`, with extra classes appended.
#[component]
pub async fn toast_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let dismiss = (!attrs.contains_key("onclick")).then_some("toast.close(this)");

    // `!` overrides the `NoShadow` button's `bg-main` and `text-main-foreground`.
    Ok(view! {
        <button
            type="button"
            data-slot="toast-action"
            onclick=(dismiss)
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Sm),
                "bg-secondary-background! text-foreground! shrink-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </button>
    })
}

/// The button that closes a [`toast`]: an "x" in a borderless icon button.
/// `attrs` are forwarded to the `<button>`, with extra classes appended.
#[component]
pub async fn toast_close(#[default] mut attrs: Attributes) -> Result<impl View> {
    // `!` overrides the `Neutral` icon button's size, border, fill, shadow and
    // hover shift.
    Ok(view! {
        <button
            type="button"
            data-slot="toast-close"
            aria-label="Close toast"
            onclick="toast.close(this)"
            class=(class!(
                button_variants(ButtonVariant::Neutral, ButtonSize::Icon),
                "size-7! border-0! bg-transparent! shadow-none! hover:translate-x-0! \
                 hover:translate-y-0! relative shrink-0 text-foreground after:absolute \
                 after:-inset-2 after:content-[''] hover:opacity-70",
                attrs.remove("class"),
            ))
            (attrs)
        >
            icon(data: iconify_icon!("lucide:x"))
        </button>
    })
}
