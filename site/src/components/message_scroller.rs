// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button};

/// Runs on the viewport's `scroll` event (`this` is the viewport). Shows each
/// [`message_scroller_button`] while its edge is more than 8px away.
///
/// The viewport is `flex-col-reverse`, so `scrollTop` is 0 at the end and
/// negative above it.
const ON_SCROLL: &str = r#"const r=this.closest('[data-slot=message-scroller]');if(!r)return;const e=Math.abs(this.scrollTop),s=this.scrollHeight-this.clientHeight-e;for(const b of r.querySelectorAll('[data-slot=message-scroller-button]')){const a=(b.dataset.direction==='start'?s:e)>8;b.dataset.active=a;b.inert=!a}"#;

/// Runs on a [`message_scroller_button`]'s `click` (`this` is the button).
/// Smoothly scrolls the viewport to the button's edge.
const ON_CLICK: &str = r#"const v=this.closest('[data-slot=message-scroller]')?.querySelector('[data-slot=message-scroller-viewport]');if(v){this.blur();v.scrollTo({top:this.dataset.direction==='start'?-v.scrollHeight:0,behavior:'smooth'})}"#;

/// Groups a [`message_scroller`] with controls outside it.
///
/// It renders only its children. The Octane version shares scroll state
/// through it; here the state lives in the DOM, so it is optional and kept
/// so ported markup reads the same.
#[component]
#[allow(dead_code)]
pub async fn message_scroller_provider(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! { (child) })
}

/// A chat transcript that starts scrolled to the newest message and stays
/// there as messages arrive.
///
/// Holds a [`message_scroller_viewport`] and an optional
/// [`message_scroller_button`]. Give it a height through its parent or
/// `class` in `attrs`.
///
/// Pinning to the end is pure CSS: the viewport is a `column-reverse` flex
/// box, which browsers anchor at the bottom. While the reader has scrolled
/// up, new messages push the transcript up instead of the view staying put.
/// The jump button uses a few lines of inline JavaScript. Scroll anchoring
/// to a chosen item (`scroll_anchor`) and the Octane hooks
/// (`useMessageScroller`, ...) are not ported.
///
/// ```ignore
/// view! {
///     message_scroller(
///         message_scroller_viewport(
///             message_scroller_content(
///                 message_scroller_item(message_id: Some("m1"), "Hello")
///             )
///         )
///         message_scroller_button()
///     )
/// }
/// ```
#[component]
pub async fn message_scroller(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-scroller"
            class=(class!(
                "group/message-scroller relative flex size-full min-h-0 flex-col overflow-hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The scrolling region of a [`message_scroller`]. Holds a
/// [`message_scroller_content`].
///
/// It is a focusable `region` named "Messages"; pass `aria-label` in `attrs`
/// to rename it.
#[component]
#[allow(dead_code)]
pub async fn message_scroller_viewport(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let label = attrs.remove("aria-label");
    Ok(view! {
        <div
            data-slot="message-scroller-viewport"
            role="region"
            match label {
                Some(label) => aria-label=(label),
                None => aria-label="Messages",
            }
            tabindex="0"
            onscroll=(ON_SCROLL)
            // `*:shrink-0` keeps the content at its full height inside the
            // `column-reverse` flex box; shrunk to the viewport, it would spill
            // its older messages out of reach instead of scrolling.
            class=(class!(
                "size-full min-h-0 min-w-0 overflow-y-auto overscroll-contain flex flex-col-reverse \
                 *:shrink-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The list of [`message_scroller_item`]s. Screen readers announce items
/// added to it.
#[component]
#[allow(dead_code)]
pub async fn message_scroller_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-scroller-content"
            role="log"
            aria-relevant="additions"
            class=(class!("flex h-max min-h-full flex-col gap-6", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One message in a [`message_scroller_content`].
///
/// Off-screen items skip rendering work (`content-visibility: auto`).
/// `message_id` is written to `data-message-id`; `scroll_anchor` to
/// `data-scroll-anchor` for styling only.
#[component]
#[allow(dead_code)]
pub async fn message_scroller_item(
    /// A stable id for the message.
    #[default]
    message_id: Option<&str>,
    /// Marks the item as a scroll anchor.
    #[default]
    scroll_anchor: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="message-scroller-item"
            data-message-id=(message_id)
            data-scroll-anchor=(if scroll_anchor { "true" } else { "false" })
            class=(class!(
                "min-w-0 shrink-0 [contain-intrinsic-size:auto_10rem] [content-visibility:auto]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Which edge a [`message_scroller_button`] jumps to.
///
/// [`Default`] is `ScrollDirection::End`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ScrollDirection {
    /// The oldest message, at the top.
    Start,
    /// The newest message, at the bottom.
    #[default]
    End,
}

impl ScrollDirection {
    /// The `data-direction` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }

    /// The button's screen-reader label.
    fn label(self) -> &'static str {
        match self {
            Self::Start => "Scroll to start",
            Self::End => "Scroll to end",
        }
    }
}

/// A round arrow [`button`] floating over a [`message_scroller`] that jumps
/// to the newest (or, with `ScrollDirection::Start`, the oldest) message.
///
/// It fades in once the reader scrolls more than 8px away from that edge.
/// `direction` defaults to `End`, `variant` to `ButtonVariant::Neutral` and
/// `size` to `ButtonSize::Icon`. A start button first appears on the first
/// scroll.
#[component]
#[allow(dead_code)]
pub async fn message_scroller_button(
    #[default] direction: ScrollDirection,
    #[default(ButtonVariant::Neutral)] variant: ButtonVariant,
    #[default(ButtonSize::Icon)] size: ButtonSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    // `size-8!` must beat the icon button's `size-10`: there is no
    // tailwind-merge to drop it.
    let class = class!(
        "absolute inset-s-1/2 size-8! rounded-full! -translate-x-1/2 \
         transition-[translate,scale,opacity] duration-200 shadow-none! hover:-translate-x-1/2! \
         hover:translate-y-0! data-[active=false]:pointer-events-none data-[active=false]:scale-95 \
         data-[active=false]:opacity-0 data-[active=false]:duration-400 \
         data-[active=false]:ease-[cubic-bezier(0.7,0,0.84,0)] data-[active=true]:translate-y-0 \
         data-[active=true]:scale-100 data-[active=true]:opacity-100 \
         data-[active=true]:ease-[cubic-bezier(0.23,1,0.32,1)] data-[direction=end]:bottom-4 \
         data-[direction=end]:data-[active=false]:translate-y-full data-[direction=start]:top-4 \
         data-[direction=start]:data-[active=false]:-translate-y-full rtl:translate-x-1/2 \
         rtl:hover:translate-x-1/2 data-[direction=start]:[&_svg]:rotate-180",
        attrs.remove("class"),
    );
    Ok(view! {
        button(
            variant: variant,
            size: size,
            attrs: attributes! {
                type="button"
                data-slot="message-scroller-button"
                data-direction=(direction.as_str())
                data-active="false"
                inert=""
                onclick=(ON_CLICK)
                class=(class)
                (attrs)
            },
            icon(data: iconify_icon!("lucide:arrow-down"))
            <span class="sr-only">(direction.label())</span>
        )
    })
}
