//! The Octane examples use the `avatar` component. These render the same
//! markup and classes with plain HTML (the `fallback` helper below).

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, Signal, shard, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    bubble::{BubbleVariant, bubble, bubble_content},
    button::{ButtonSize, ButtonVariant, button},
    message::{MessageAlign, message, message_avatar, message_content},
    message_scroller::{
        message_scroller, message_scroller_button, message_scroller_content,
        message_scroller_item, message_scroller_provider, message_scroller_viewport,
    },
};

/// An avatar showing initials, styled like `avatar` + `avatar_fallback`.
#[component]
async fn fallback(initials: &str) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar"
            class="group/avatar relative flex size-10 shrink-0 rounded-full outline-2 outline-border"
        >
            <span
                data-slot="avatar-fallback"
                class="flex size-full items-center justify-center rounded-full \
                    bg-secondary-background text-foreground font-base"
            >
                (initials)
            </span>
        </span>
    })
}

/// One chat line, aligned and colored by who sent it.
#[component]
async fn chat_message(index: usize, from_me: bool, text: &str) -> Result<impl View> {
    let id = format!("message-{index}");
    Ok(view! {
        message_scroller_item(message_id: Some(&id),
            message(align: if from_me { MessageAlign::End } else { MessageAlign::Start },
                message_avatar(fallback(initials: "BF"))
                message_content(
                    bubble(
                        variant: if from_me { BubbleVariant::Default } else { BubbleVariant::Muted },
                        bubble_content((text))
                    )
                )
            )
        )
    })
}

/// The chat's title bar, with a reset button.
#[component]
async fn chat_header(#[default] reset: Option<Signal<usize>>) -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center gap-3 p-4">
            <div class="flex min-w-0 flex-1 flex-col gap-0.5 text-sm">
                <span class="truncate font-heading">"New Chat"</span>
                <span class="truncate font-base">"How can I help you today?"</span>
            </div>
            match reset {
                Some(count) => button(
                    variant: ButtonVariant::Neutral,
                    size: ButtonSize::Icon,
                    attrs: attributes! {
                        class="size-9! shadow-none!"
                        aria-label="Reset chat"
                        @click=$(|_e: Event| count.set(INITIAL))
                    },
                    icon(data: iconify_icon!("lucide:rotate-cw"))
                ),
                None => button(
                    variant: ButtonVariant::Neutral,
                    size: ButtonSize::Icon,
                    attrs: attributes! { class="size-9! shadow-none!" aria-label="Reset chat" },
                    icon(data: iconify_icon!("lucide:rotate-cw"))
                ),
            }
        </div>
        <div class="h-0.5 w-full shrink-0 bg-border"></div>
    })
}

/// `(from_me, text)` for each message of the demo.
const MESSAGES: [(bool, &str); 12] = [
    (true, "Deploying to prod real quick."),
    (false, "It's 4:55 PM. On a Friday."),
    (true, "Ship it."),
    (false, "Did the registry build go through?"),
    (true, "Yes, one stale route showed up though."),
    (false, "Which one?"),
    (true, "The old toast page, still in the sidebar."),
    (false, "Remove it and rebuild."),
    (true, "Done. Lint and tsc are clean."),
    (false, "Charts render fine with recharts 3?"),
    (true, "They do. Pushing the fix now."),
    (false, "Perfect, that is exactly what I had in mind."),
];

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div
            class="flex h-[360px] w-full max-w-md flex-col overflow-hidden rounded-base border-2 \
                border-border bg-secondary-background text-foreground"
        >
            chat_header()
            message_scroller_provider(
                message_scroller(attrs: attributes! { class="p-4" },
                    message_scroller_viewport(
                        message_scroller_content(attrs: attributes! { class="gap-4" },
                            #[key(index)]
                            for (index, (from_me, text)) in MESSAGES.into_iter().enumerate() {
                                chat_message(index: index, from_me: from_me, text: text)
                            }
                        )
                    )
                    message_scroller_button()
                )
            )
        </div>
    })
}

/// The first messages of the add-messages chat, as `(from_me, text)`.
const INITIAL_MESSAGES: [(bool, &str); 3] = [
    (false, "Hey, are you around?"),
    (true, "Yes, what is up?"),
    (false, "I need a review on the message scroller."),
];

/// How many messages the add-messages chat starts with.
const INITIAL: usize = INITIAL_MESSAGES.len();

/// Replies added in turn by "Add message".
const REPLIES: [&str; 5] = [
    "Sounds good to me.",
    "Can you double check the build output?",
    "Done, everything looks clean.",
    "Let us ship it then.",
    "Pushing now.",
];

/// Scrolls the chat to its newest message (the viewport is `column-reverse`,
/// so the end is `top: 0`), before the shard re-renders with the new one.
const SCROLL_TO_END: &str = "document.querySelector('#add-messages \
    [data-slot=message-scroller-viewport]')?.scrollTo({top:0,behavior:'smooth'})";

/// The add-messages chat. A shard, so changing `count` re-renders only this
/// chat on the server, with one more (or, on reset, the initial) message.
#[shard]
async fn add_messages_chat(cx: &Cx) -> Result<impl View> {
    let count = signal(cx, || INITIAL);
    // The browser sends `count` back on every re-render: keep it in range.
    let shown = count.get().clamp(INITIAL, 100);
    // Messages alternate senders; the first added one is from me.
    let messages = (0..shown).map(|index| match INITIAL_MESSAGES.get(index) {
        Some(&line) => line,
        None => ((index - INITIAL) % 2 == 0, REPLIES[(index - INITIAL) % REPLIES.len()]),
    });

    Ok(view! {
        <div id="add-messages" class="flex w-full max-w-md flex-col gap-4">
            <div
                class="flex h-[360px] w-full flex-col overflow-hidden rounded-base border-2 \
                    border-border bg-secondary-background text-foreground"
            >
                chat_header(reset: Some(count.clone()))
                message_scroller(attrs: attributes! { class="p-4" },
                    message_scroller_viewport(
                        message_scroller_content(attrs: attributes! { class="gap-4" },
                            #[key(index)]
                            for (index, (from_me, text)) in messages.enumerate() {
                                chat_message(index: index, from_me: from_me, text: text)
                            }
                        )
                    )
                    message_scroller_button()
                )
            </div>
            <div class="flex justify-end">
                button(
                    attrs: attributes! {
                        onclick=(SCROLL_TO_END)
                        @click=$(|_e: Event| count.increment())
                    },
                    "Add message"
                )
            </div>
        </div>
    })
}

#[component]
pub async fn add_messages() -> Result<impl View> {
    Ok(view! { add_messages_chat() })
}
