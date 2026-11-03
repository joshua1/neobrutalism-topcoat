// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button};

/// The upload state of an [`attachment`].
///
/// [`Default`] is `AttachmentState::Done`, used when no state is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AttachmentState {
    /// Waiting for a file: a dashed border.
    Idle,
    /// Uploading: the title pulses and an image preview is dimmed.
    Uploading,
    /// Processing after upload: the title pulses and an image preview is dimmed.
    Processing,
    /// Failed: black with white text.
    Error,
    /// Uploaded.
    #[default]
    Done,
}

impl AttachmentState {
    /// The `data-state` value the attachment's parts style themselves by.
    fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Uploading => "uploading",
            Self::Processing => "processing",
            Self::Error => "error",
            Self::Done => "done",
        }
    }
}

/// The size of an [`attachment`].
///
/// [`Default`] is `AttachmentSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AttachmentSize {
    /// The standard size.
    #[default]
    Default,
    /// A compact attachment with smaller text.
    Sm,
    /// An extra-compact attachment.
    Xs,
}

impl AttachmentSize {
    /// Classes for the attachment's gap, text size and padding.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!(
                "gap-2 text-sm has-data-[slot=attachment-content]:px-2.5 \
                 has-data-[slot=attachment-content]:py-2 has-data-[slot=attachment-media]:p-2",
            ),
            Self::Sm => class!(
                "gap-2.5 text-xs has-data-[slot=attachment-content]:px-2 \
                 has-data-[slot=attachment-content]:py-1.5 has-data-[slot=attachment-media]:p-1.5",
            ),
            Self::Xs => class!(
                "gap-1.5 text-xs has-data-[slot=attachment-content]:px-1.5 \
                 has-data-[slot=attachment-content]:py-1 has-data-[slot=attachment-media]:p-1",
            ),
        }
    }

    /// The `data-size` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Xs => "xs",
        }
    }
}

/// The layout of an [`attachment`].
///
/// [`Default`] is `AttachmentOrientation::Horizontal`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AttachmentOrientation {
    /// Media beside the content, in a row.
    #[default]
    Horizontal,
    /// A tile with the media above the content and actions in the corner.
    Vertical,
}

impl AttachmentOrientation {
    /// Classes for the layout.
    fn classes(self) -> StaticClass {
        match self {
            Self::Horizontal => class!("min-w-40 items-center"),
            Self::Vertical => class!("w-24 flex-col has-data-[slot=attachment-content]:w-30"),
        }
    }

    /// The `data-orientation` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Classes shared by every attachment.
const ATTACHMENT: StaticClass = class!(
    "group/attachment relative flex w-fit max-w-full min-w-0 shrink-0 flex-wrap rounded-base \
     border-2 border-border bg-background text-foreground font-base transition-colors \
     focus-within:ring-2 focus-within:ring-black focus-within:ring-offset-2 \
     has-[>a,>button]:hover:bg-secondary-background data-[state=error]:bg-black \
     data-[state=error]:text-white data-[state=idle]:border-dashed",
);

/// A file attached to a message or form: a preview, its name and actions.
///
/// `state` defaults to `Done`, `size` to `Default` and `orientation` to
/// `Horizontal`. `attrs` are forwarded to the `<div>`, with extra classes
/// appended to its classes. Add an [`attachment_trigger`] to make the whole
/// attachment clickable.
///
/// ```ignore
/// view! {
///     attachment(state: AttachmentState::Uploading,
///         attachment_media(icon(data: iconify_icon!("lucide:file-text")))
///         attachment_content(
///             attachment_title("report.pdf")
///             attachment_description("Uploading · 45%")
///         )
///         attachment_actions(
///             attachment_action(
///                 attrs: attributes! { aria-label="Cancel" },
///                 icon(data: iconify_icon!("lucide:x"))
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn attachment(
    #[default] state: AttachmentState,
    #[default] size: AttachmentSize,
    #[default] orientation: AttachmentOrientation,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="attachment"
            data-state=(state.as_str())
            data-size=(size.as_str())
            data-orientation=(orientation.as_str())
            class=(class!(
                ATTACHMENT,
                size.classes(),
                orientation.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// What an [`attachment_media`] holds.
///
/// [`Default`] is `AttachmentMediaVariant::Icon`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AttachmentMediaVariant {
    /// A file-type icon or spinner.
    #[default]
    Icon,
    /// An image preview, dimmed until the upload is done.
    Image,
}

impl AttachmentMediaVariant {
    /// Classes for the media variant, or `None` for an icon.
    fn classes(self) -> Option<StaticClass> {
        match self {
            Self::Icon => None,
            Self::Image => Some(class!(
                "opacity-60 group-data-[state=done]/attachment:opacity-100 \
                 group-data-[state=idle]/attachment:opacity-100 *:[img]:aspect-square \
                 *:[img]:w-full *:[img]:object-cover",
            )),
        }
    }

    /// The `data-variant` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Icon => "icon",
            Self::Image => "image",
        }
    }
}

/// The square preview of an [`attachment`]: an icon or an `<img>`.
#[component]
#[allow(dead_code)]
pub async fn attachment_media(
    #[default] variant: AttachmentMediaVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="attachment-media"
            data-variant=(variant.as_str())
            class=(class!(
                "relative flex aspect-square w-10 shrink-0 items-center justify-center \
                 overflow-hidden rounded-base border-2 border-border bg-secondary-background \
                 text-foreground group-data-[orientation=vertical]/attachment:w-full \
                 group-data-[size=sm]/attachment:w-8 group-data-[size=xs]/attachment:w-7 \
                 group-data-[state=error]/attachment:text-black \
                 group-data-[orientation=vertical]/attachment:*:data-[slot=spinner]:size-6! \
                 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 \
                 group-data-[orientation=vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 \
                 group-data-[size=xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5",
                variant.classes(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The title and description column of an [`attachment`].
#[component]
#[allow(dead_code)]
pub async fn attachment_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="attachment-content"
            class=(class!(
                "max-w-full min-w-0 flex-1 leading-tight \
                 group-data-[orientation=vertical]/attachment:px-1",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The file name, truncated to one line. Pulses while uploading or processing.
#[component]
#[allow(dead_code)]
pub async fn attachment_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="attachment-title"
            class=(class!(
                "block max-w-full min-w-0 truncate font-heading \
                 group-data-[state=processing]/attachment:animate-pulse \
                 group-data-[state=uploading]/attachment:animate-pulse",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// A short line under the title, such as the file type and size.
#[component]
#[allow(dead_code)]
pub async fn attachment_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="attachment-description"
            class=(class!(
                "mt-0.5 block min-w-0 truncate text-xs text-foreground \
                 group-data-[state=error]/attachment:text-white",
                "max-w-full",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// The [`attachment_action`] buttons of an [`attachment`]. In a vertical
/// attachment they sit in the top-right corner.
#[component]
#[allow(dead_code)]
pub async fn attachment_actions(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="attachment-actions"
            class=(class!(
                "relative z-20 flex shrink-0 items-center \
                 group-data-[orientation=vertical]/attachment:absolute \
                 group-data-[orientation=vertical]/attachment:top-3 \
                 group-data-[orientation=vertical]/attachment:right-3 \
                 group-data-[orientation=vertical]/attachment:gap-1",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A small flat icon [`button`] in [`attachment_actions`], such as remove.
///
/// `variant` defaults to `ButtonVariant::Neutral` and `size` to
/// `ButtonSize::Icon`. Give it an `aria-label` in `attrs`.
#[component]
#[allow(dead_code)]
pub async fn attachment_action(
    #[default(ButtonVariant::Neutral)] variant: ButtonVariant,
    #[default(ButtonSize::Icon)] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // `!` on the utilities that clash with the button's own (`size-10`,
    // `border-2`, its fill and text color): there is no tailwind-merge to drop
    // those, so these must win by importance.
    let class = class!(
        "size-6! border-0! bg-transparent! text-inherit! shadow-none! hover:translate-x-0! \
         hover:translate-y-0!",
        attrs.remove("class"),
    );
    Ok(view! {
        button(
            variant: variant,
            size: size,
            attrs: attributes! { data-slot="attachment-action" class=(class) (attrs) },
            (child)
        )
    })
}

/// An invisible button covering the whole [`attachment`], to open or preview
/// the file. Give it an `aria-label` and a handler in `attrs`.
#[component]
#[allow(dead_code)]
pub async fn attachment_trigger(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="attachment-trigger"
            class=(class!("absolute inset-0 z-10 outline-none", attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}
