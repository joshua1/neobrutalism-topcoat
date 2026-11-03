//! A dialog that interrupts the user with an important message and asks for a
//! response.
//!
//! It is a [`dialog`] with `role="alertdialog"` whose overlay ignores clicks,
//! so the user has to choose an action (Escape still cancels). The portal and
//! overlay parts of Base UI are the `<dialog>` element itself; see
//! [`dialog`](super::dialog) for how opening, closing and focus work.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    runtime::Expr,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button_variants};
use super::dialog::{DIALOG_POPUP_MOTION, dialog, dialog_close, dialog_trigger};

/// A modal alert dialog: the overlay that holds an [`alert_dialog_content`].
///
/// Give it an `id` and open it with an [`alert_dialog_trigger`] naming that id,
/// or pass a runtime expression to `open` (see [`dialog`]). Point
/// `aria-labelledby` and `aria-describedby` in `attrs` at the title and
/// description ids. Other attributes are forwarded to the `<dialog>`.
///
/// ```ignore
/// view! {
///     alert_dialog_trigger(
///         target: "delete-account",
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
///         },
///         "Delete account"
///     )
///     alert_dialog(
///         id: "delete-account",
///         alert_dialog_content(
///             alert_dialog_header(
///                 alert_dialog_title("Are you absolutely sure?")
///                 alert_dialog_description("This action cannot be undone.")
///             )
///             alert_dialog_footer(
///                 alert_dialog_cancel("Cancel")
///                 alert_dialog_action("Continue")
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn alert_dialog(
    /// Whether the alert dialog shows without being opened by a trigger.
    #[into]
    #[default(Expr::from(false))]
    open: Expr<bool>,
    /// The `id` that [`alert_dialog_trigger`]'s `target` refers to.
    #[into]
    #[default]
    id: Option<&str>,
    /// Extra attributes for the `<dialog>` element.
    #[default]
    attrs: Attributes,
    /// The alert dialog's content, usually an [`alert_dialog_content`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        dialog(
            open: open,
            id: id,
            disable_pointer_dismissal: true,
            attrs: attributes! { data-slot="alert-dialog" role="alertdialog" (attrs) },
            (child)
        )
    })
}

/// A button that opens the [`alert_dialog`] whose `id` is `target`.
///
/// It is unstyled; give it button styling with `class=(button_variants(..))`.
#[component]
pub async fn alert_dialog_trigger(
    /// The `id` of the alert dialog to open.
    target: &str,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        dialog_trigger(
            target: target,
            attrs: attributes! { data-slot="alert-dialog-trigger" (attrs) },
            (child)
        )
    })
}

/// The width of an [`alert_dialog_content`].
///
/// [`Default`] is `AlertDialogSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AlertDialogSize {
    /// Up to `max-w-lg` on wider screens.
    #[default]
    Default,
    /// A narrow, centered layout with side-by-side actions.
    Sm,
}

impl AlertDialogSize {
    /// The value of the panel's `data-size` attribute, which the header, footer
    /// and media read through the `group/alert-dialog-content` group.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }
}

/// Classes for the centered alert dialog panel.
const CONTENT: StaticClass = class!(
    "group/alert-dialog-content bg-background fixed top-[50%] left-[50%] z-50 grid w-full \
     max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] gap-4 rounded-base border-2 \
     border-border p-6 shadow-shadow duration-200 outline-none sm:max-w-lg \
     sm:data-[size=sm]:max-w-sm",
);

/// The panel inside an [`alert_dialog`]. It has no close button; close it with
/// [`alert_dialog_cancel`] or [`alert_dialog_action`].
#[component]
pub async fn alert_dialog_content(
    /// The panel's width and layout.
    #[default]
    size: AlertDialogSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-dialog-content"
            data-size=(size.as_str())
            class=(class!(CONTENT, DIALOG_POPUP_MOTION, attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The opening section of an [`alert_dialog_content`]: optional media, the
/// title and the description.
#[component]
pub async fn alert_dialog_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-dialog-header"
            class=(class!(
                "flex flex-col gap-2 text-center sm:text-left \
                 group-data-[size=sm]/alert-dialog-content:text-center",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The row of actions at the end of an [`alert_dialog_content`]. In the small
/// size the actions sit side by side in two equal columns.
#[component]
pub async fn alert_dialog_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-dialog-footer"
            class=(class!(
                "flex flex-col-reverse gap-3 sm:flex-row sm:justify-end \
                 group-data-[size=sm]/alert-dialog-content:grid \
                 group-data-[size=sm]/alert-dialog-content:grid-cols-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A boxed icon shown above the title of an [`alert_dialog_content`].
#[component]
pub async fn alert_dialog_media(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="alert-dialog-media"
            class=(class!(
                "mb-2 flex size-12 shrink-0 items-center justify-center self-center rounded-base \
                 border-2 border-border bg-main text-main-foreground [&_svg]:size-6 sm:size-10 \
                 sm:self-start sm:[&_svg]:size-5 \
                 group-data-[size=sm]/alert-dialog-content:self-center",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of an [`alert_dialog`], rendered as an `<h2>`.
#[component]
pub async fn alert_dialog_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="alert-dialog-title"
            class=(class!("text-lg font-heading", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// The message of an [`alert_dialog`], rendered as a `<p>`.
#[component]
pub async fn alert_dialog_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="alert-dialog-description"
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}

/// The confirming button of an [`alert_dialog`]. It closes the dialog.
///
/// Styled as a [`button`](super::button::button); `variant` and `size` default
/// to `Default`. To act on the choice, wrap the content in a
/// `<form method="dialog">` and read the dialog's `returnValue`, or add your own
/// `onclick` / runtime handler next to the close.
#[component]
pub async fn alert_dialog_action(
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = attrs.remove("class");

    Ok(view! {
        dialog_close(
            attrs: attributes! {
                data-slot="alert-dialog-action"
                class=(class!(button_variants(variant, size), class))
                (attrs)
            },
            (child)
        )
    })
}

/// The cancelling button of an [`alert_dialog`]. It closes the dialog.
///
/// Styled as a [`button`](super::button::button); `variant` defaults to
/// `Neutral` and `size` to `Default`.
#[component]
pub async fn alert_dialog_cancel(
    #[default(ButtonVariant::Neutral)] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let class = attrs.remove("class");

    Ok(view! {
        dialog_close(
            attrs: attributes! {
                data-slot="alert-dialog-cancel"
                class=(class!(button_variants(variant, size), class))
                (attrs)
            },
            (child)
        )
    })
}
