//! A modal dialog built on the native `<dialog>` element.
//!
//! Base UI's `Dialog` root, `DialogPortal` and `DialogOverlay` have no separate
//! parts here. The [`dialog`] element *is* the overlay: it covers the viewport
//! with the overlay color, and `showModal()` lifts it into the browser's top
//! layer, which takes the place of the portal.
//!
//! The dialog opens and closes without the Topcoat runtime:
//!
//! - [`dialog_trigger`] opens it with the `commandfor` / `command="show-modal"`
//!   invoker commands, with a one-line `onclick` fallback for browsers that
//!   lack them.
//! - [`dialog_close`] (and the corner button of [`dialog_content`]) closes the
//!   nearest `<dialog>`.
//! - Escape closes it, and a click on the overlay closes it unless
//!   `disable_pointer_dismissal` is set.
//!
//! A modal dialog makes the rest of the page inert, so focus stays inside it.
//! Unlike Base UI, the page behind it still scrolls; add
//! `html:has(dialog:modal) { overflow: hidden }` to your styles to lock it.
//!
//! Other dialog-like components ([`alert_dialog`](super::alert_dialog),
//! [`sheet`](super::sheet), [`drawer`](super::drawer), `command`) reuse
//! [`dialog_trigger`], [`dialog_close`] and the script constants below.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    runtime::Expr,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

/// Inline `onclick` for a button with `commandfor` and `command="show-modal"`.
///
/// Browsers with invoker commands open the dialog from the attributes alone;
/// this opens it in the ones without them, and does nothing in the others.
#[allow(dead_code)]
pub const OPEN_MODAL_FALLBACK: &str = "if(!('command' in this))\
     document.getElementById(this.getAttribute('commandfor')).showModal()";

/// Inline `onclick` that opens the dialog named by the button's
/// `aria-controls` without making it modal, so the page stays interactive.
#[allow(dead_code)]
pub const SHOW_SCRIPT: &str = "document.getElementById(this.getAttribute('aria-controls')).show()";

/// Inline `onclick` that closes the `<dialog>` containing the element.
#[allow(dead_code)]
pub const CLOSE_SCRIPT: &str = "this.closest('dialog').close()";

/// Inline `onclick` for a `<dialog>` that closes it when the click lands on the
/// dialog itself (the overlay) rather than on its content.
#[allow(dead_code)]
pub const LIGHT_DISMISS_SCRIPT: &str = "if(event.target===this)this.close()";

/// Classes that turn a `<dialog>` into a viewport-sized overlay, without its
/// color or transition. It names the `group/dialog` group, which content panels
/// use to style themselves by the dialog's `open` state.
///
/// Display is left to the browser so a closed dialog stays hidden.
#[allow(dead_code)]
pub const DIALOG_OVERLAY: StaticClass = class!(
    "group/dialog fixed inset-0 z-50 size-full max-h-none max-w-none text-foreground \
     backdrop:bg-transparent",
);

/// Classes that fade the overlay in and out.
///
/// `transition-discrete` keeps it displayed, and in the top layer, until the
/// exit transition ends. `starting:` (`@starting-style`) supplies the entry
/// opacity.
const FADE: StaticClass = class!(
    "bg-overlay opacity-0 open:opacity-100 starting:open:opacity-0 \
     transition-[opacity,display,overlay] transition-discrete duration-200 ease-out",
);

/// Classes that fade and zoom a centered panel as its [`dialog`] opens or closes.
#[allow(dead_code)]
pub const DIALOG_POPUP_MOTION: StaticClass = class!(
    "scale-95 opacity-0 group-open/dialog:scale-100 group-open/dialog:opacity-100 \
     starting:group-open/dialog:scale-95 starting:group-open/dialog:opacity-0 \
     transition-[scale,opacity] ease-out",
);

/// Returns `value` for the attribute `key`, unless the caller set it in `attrs`.
///
/// Lets a wrapping component (such as `alert_dialog`) replace `data-slot`
/// without rendering the attribute twice.
fn unless_given(attrs: &Attributes, key: &str, value: &'static str) -> Option<&'static str> {
    (!attrs.contains_key(key)).then_some(value)
}

/// A modal dialog: the overlay that holds a [`dialog_content`] panel.
///
/// Give it an `id` and open it with a [`dialog_trigger`] naming that id. No
/// runtime is needed. To control it from the Topcoat runtime instead, pass a
/// runtime expression to `open`. The `open` attribute shows the dialog without
/// the top layer (it still covers the page), so keep the signal in sync when the
/// browser closes it:
///
/// ```ignore
/// let editing = signal(cx, || false);
/// view! {
///     dialog(
///         open: $(editing.get()),
///         attrs: attributes! { @close=$(|_e| editing.set(false)) },
///         dialog_content(..)
///     )
/// }
/// ```
///
/// Clicking the overlay closes the dialog unless `disable_pointer_dismissal` is
/// set. Name the dialog with `aria-labelledby` pointing at its title's `id`.
/// `attrs` are forwarded to the `<dialog>`, with extra classes appended.
///
/// ```ignore
/// view! {
///     dialog_trigger(
///         target: "edit-profile",
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
///         },
///         "Edit profile"
///     )
///     dialog(
///         id: "edit-profile",
///         attrs: attributes! { aria-labelledby="edit-profile-title" },
///         dialog_content(
///             dialog_header(
///                 dialog_title(attrs: attributes! { id="edit-profile-title" }, "Edit profile")
///                 dialog_description("Make changes to your profile here.")
///             )
///             dialog_footer(
///                 dialog_close(
///                     attrs: attributes! {
///                         class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
///                     },
///                     "Cancel"
///                 )
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn dialog(
    /// Whether the dialog shows without being opened by a trigger. A runtime
    /// expression keeps it in sync with a signal.
    #[into]
    #[default(Expr::from(false))]
    open: Expr<bool>,
    /// The `id` that [`dialog_trigger`]'s `target` refers to.
    #[into]
    #[default]
    id: Option<&str>,
    /// Keeps a click on the overlay from closing the dialog.
    #[default]
    disable_pointer_dismissal: bool,
    /// Extra attributes for the `<dialog>` element.
    #[default]
    mut attrs: Attributes,
    /// The dialog's content, usually a [`dialog_content`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let slot = unless_given(&attrs, "data-slot", "dialog");
    let dismiss = (!disable_pointer_dismissal).then_some(LIGHT_DISMISS_SCRIPT);

    Ok(view! {
        <dialog
            data-slot=(slot)
            id=(id)
            :open=(open)
            onclick=(dismiss)
            class=(class!(DIALOG_OVERLAY, FADE, attrs.remove("class")))
            (attrs)
        >
            (child)
        </dialog>
    })
}

/// A button that opens the [`dialog`] whose `id` is `target`.
///
/// It is unstyled; give it button styling with `class=(button_variants(..))` in
/// `attrs`. A `data-slot` in `attrs` replaces `dialog-trigger`, so other
/// dialog-like components can reuse it.
#[component]
pub async fn dialog_trigger(
    /// The `id` of the dialog to open.
    target: &str,
    /// Extra attributes for the `<button>` element.
    #[default]
    attrs: Attributes,
    /// The button's label.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let slot = unless_given(&attrs, "data-slot", "dialog-trigger");

    Ok(view! {
        <button
            type="button"
            data-slot=(slot)
            aria-haspopup="dialog"
            commandfor=(target)
            command="show-modal"
            onclick=(OPEN_MODAL_FALLBACK)
            (attrs)
        >
            (child)
        </button>
    })
}

/// A button that closes the `<dialog>` it is inside.
///
/// It is unstyled; give it button styling with `class=(button_variants(..))` in
/// `attrs`. A `data-slot` in `attrs` replaces `dialog-close`.
#[component]
pub async fn dialog_close(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let slot = unless_given(&attrs, "data-slot", "dialog-close");

    Ok(view! {
        <button type="button" data-slot=(slot) onclick=(CLOSE_SCRIPT) (attrs)>
            (child)
        </button>
    })
}

/// Classes for the centered dialog panel.
const CONTENT: StaticClass = class!(
    "bg-background fixed top-[50%] left-[50%] z-50 grid w-full max-w-[calc(100%-2rem)] \
     translate-x-[-50%] translate-y-[-50%] gap-4 rounded-base border-2 border-border p-6 \
     shadow-shadow duration-200 outline-none sm:max-w-lg",
);

/// The panel inside a [`dialog`], with a close button in its corner.
///
/// Set `show_close_button: false` to leave the corner button out. To change
/// the panel's maximum width, use an important class such as
/// `sm:max-w-[425px]!` so it wins over the default `sm:max-w-lg`.
#[component]
pub async fn dialog_content(
    /// Whether to render the close button in the top-right corner.
    #[default(true)]
    show_close_button: bool,
    /// Extra attributes for the panel `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The panel's sections.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dialog-content"
            class=(class!(CONTENT, DIALOG_POPUP_MOTION, attrs.remove("class")))
            (attrs)
        >
            (child)
            if show_close_button {
                dialog_close(
                    attrs: attributes! {
                        class="absolute right-4 top-4 rounded-base opacity-100 ring-offset-white \
                            focus:outline-hidden focus:ring-2 focus:ring-black focus:ring-offset-2 \
                            disabled:pointer-events-none [&_svg]:pointer-events-none \
                            [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
                    },
                    icon(data: iconify_icon!("lucide:x"))
                    <span class="sr-only">"Close"</span>
                )
            }
        </div>
    })
}

/// The opening section of a [`dialog_content`], stacking a [`dialog_title`]
/// and a [`dialog_description`].
#[component]
pub async fn dialog_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dialog-header"
            class=(class!("flex flex-col gap-2 text-center sm:text-left", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The row of actions at the end of a [`dialog_content`]. Stacks on small
/// screens, with the first action at the bottom.
#[component]
pub async fn dialog_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="dialog-footer"
            class=(class!(
                "flex flex-col-reverse gap-3 sm:flex-row sm:justify-end",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`dialog`], rendered as an `<h2>`. Give it an `id` and
/// point the dialog's `aria-labelledby` at it.
#[component]
pub async fn dialog_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="dialog-title"
            class=(class!("text-lg font-heading leading-none tracking-tight", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// Text that explains the dialog's purpose, rendered as a `<p>`.
#[component]
pub async fn dialog_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="dialog-description"
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}
