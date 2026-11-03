// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// Which side of the conversation a [`message`] sits on.
///
/// [`Default`] is `MessageAlign::Start`, used when no alignment is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageAlign {
    /// The start side, for other people's messages.
    #[default]
    Start,
    /// The end side, for your own messages. The avatar moves to the end.
    End,
}

impl MessageAlign {
    /// The `data-align` value the message's parts style themselves by.
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

/// A stack of consecutive [`message`]s from the same sender.
#[component]
#[allow(dead_code)]
pub async fn message_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-group"
            class=(class!("flex min-w-0 flex-col gap-2", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A row in a conversation: an avatar beside the message content.
///
/// `align` defaults to `Start`. With `MessageAlign::End` the row reverses and
/// its content, including any `bubble`, lines up at the end.
///
/// ```ignore
/// view! {
///     message(align: MessageAlign::End,
///         message_avatar("ME")
///         message_content(
///             bubble(bubble_content("Ship it."))
///         )
///     )
/// }
/// ```
#[component]
pub async fn message(
    #[default] align: MessageAlign,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message"
            data-align=(align.as_str())
            class=(class!(
                "group/message relative flex w-full min-w-0 gap-2 text-sm font-base \
                 data-[align=end]:flex-row-reverse",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The round avatar frame of a [`message`]. Holds an `avatar`, an image or
/// initials.
#[component]
#[allow(dead_code)]
pub async fn message_avatar(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-avatar"
            class=(class!(
                "flex size-10 shrink-0 items-center justify-center self-start overflow-hidden \
                 rounded-full border-2 border-border bg-background \
                 [&_[data-slot=avatar]]:size-full [&_[data-slot=avatar]]:outline-0 \
                 [&_[data-slot=avatar-fallback]]:bg-background",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The column of bubbles, header, footer and actions in a [`message`].
#[component]
#[allow(dead_code)]
pub async fn message_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-content"
            class=(class!(
                "flex w-full min-w-0 flex-col gap-2.5 wrap-break-word \
                 group-data-[align=end]/message:*:data-slot:self-end",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A small line above a [`message`]'s bubbles, such as the sender's name.
#[component]
#[allow(dead_code)]
pub async fn message_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-header"
            class=(class!(
                "flex max-w-full min-w-0 items-center px-3 text-xs font-base text-foreground \
                 group-has-data-[variant=ghost]/message:px-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A small line below a [`message`]'s bubbles, such as the time it was sent.
#[component]
#[allow(dead_code)]
pub async fn message_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-footer"
            class=(class!(
                "flex max-w-full min-w-0 items-center px-3 text-xs font-base text-foreground \
                 group-has-data-[variant=ghost]/message:px-0 \
                 group-data-[align=end]/message:justify-end",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
