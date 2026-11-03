//! A carousel built on a native scroll container with CSS scroll snapping,
//! replacing Embla.
//!
//! [`carousel_content`] is the scroll viewport. Touch swipes, trackpads and
//! the scroll wheel move it, and `scroll-snap` settles it on a slide. The
//! [`carousel_previous`] and [`carousel_next`] buttons scroll it by one slide
//! with a line of inline JavaScript, and a scroll handler disables them at
//! the ends. ArrowLeft and ArrowRight inside the carousel press them.
//!
//! [`carousel`] sets `data-orientation` and the parts read it with
//! `group-data-[orientation=..]/carousel:` classes, so only the root takes an
//! `orientation`.
//!
//! Differences from Embla: there is no mouse dragging (use the wheel or
//! trackpad), no looping or plugins, and slides snap to their end edge, which
//! for equal-width slides is the same set of positions as Embla's `start`
//! alignment. Octane's `setApi` is replaced by `on_select`, a script that
//! runs when the current slide changes.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::button::{ButtonSize, ButtonVariant, button};

/// The direction a [`carousel`] scrolls in.
///
/// [`Default`] is `CarouselOrientation::Horizontal`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CarouselOrientation {
    /// Slides sit in a row.
    #[default]
    Horizontal,
    /// Slides sit in a column. Give [`carousel_content`] a height.
    Vertical,
}

impl CarouselOrientation {
    /// The root's `data-orientation` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Inline script, for the viewport's `onscroll` and the root's
/// `onpointerenter`, that runs on the carousel root `c` and its viewport `v`.
/// It disables the buttons at the ends and, when the slide changes, stores
/// it in the root's `data-index` and `data-count` and fires a `change` event
/// on the root, which runs `on_select`.
///
/// A step is one slide's size. Slides snap to their end edge, so the snap
/// positions are whole steps from the start: `count` is the number of
/// positions, which is less than the number of slides when several show at
/// once.
const UPDATE: &str = "const c=this.closest('[data-slot=carousel]');if(!c)return;\
     const v=c.querySelector('[data-slot=carousel-content]'),y=c.dataset.orientation==='vertical',\
     i=v.querySelector('[data-slot=carousel-item]'),p=y?v.scrollTop:v.scrollLeft,\
     m=y?v.scrollHeight-v.clientHeight:v.scrollWidth-v.clientWidth,\
     s=i?(y?i.offsetHeight:i.offsetWidth):1,b=c.querySelector('[data-slot=carousel-previous]'),\
     n=c.querySelector('[data-slot=carousel-next]');if(b)b.disabled=p<=1;if(n)n.disabled=p>=m-1;\
     const index=Math.round(p/s),count=Math.round(m/s)+1;\
     if(c.dataset.index==index&&c.dataset.count==count)return;\
     c.dataset.index=index;c.dataset.count=count;c.dispatchEvent(new Event('change'))";

/// Inline `onclick` for the previous and next buttons: scrolls the viewport
/// by one slide, backward for previous.
const SCROLL_BY_ONE: &str = "const c=this.closest('[data-slot=carousel]'),\
     v=c.querySelector('[data-slot=carousel-content]'),\
     i=v.querySelector('[data-slot=carousel-item]'),\
     d=this.dataset.slot==='carousel-next'?1:-1;if(!i)return;\
     c.dataset.orientation==='vertical'?v.scrollBy({top:d*i.offsetHeight,behavior:'smooth'})\
     :v.scrollBy({left:d*i.offsetWidth,behavior:'smooth'})";

/// Inline `onkeydown` for the root: ArrowLeft and ArrowRight press the
/// previous and next buttons (a disabled button ignores the click).
const ARROW_KEYS: &str = "if(event.key!=='ArrowLeft'&&event.key!=='ArrowRight')return;\
     event.preventDefault();this.querySelector(event.key==='ArrowLeft'\
     ?'[data-slot=carousel-previous]':'[data-slot=carousel-next]')?.click()";

/// A carousel of slides. Pass a [`carousel_content`] holding
/// [`carousel_item`]s, and optionally [`carousel_previous`] and
/// [`carousel_next`], as children.
///
/// `orientation` defaults to `Horizontal`. `on_select` is JavaScript run when
/// the current slide changes, with `index` (from 0), `count` (the number of
/// scroll positions) and `this` (the root) in scope; use it the way Octane's
/// `setApi` example does:
///
/// ```ignore
/// view! {
///     carousel(
///         on_select: Some("document.getElementById('status').textContent=`Slide ${index+1} of ${count}`"),
///         attrs: attributes! { class="w-full max-w-xs" },
///         carousel_content(
///             for index in 1..=5 {
///                 carousel_item(<div class="p-1">(index)</div>)
///             }
///         )
///         carousel_previous()
///         carousel_next()
///     )
///     <p id="status">"Slide 1 of 5"</p>
/// }
/// ```
///
/// `attrs` are forwarded to the root `<div>`, with extra classes appended.
#[component]
pub async fn carousel(
    #[default] orientation: CarouselOrientation,
    #[default] on_select: Option<&str>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    // `change` events from form controls in the slides bubble up here too;
    // only the carousel's own one runs the hook.
    let on_change = on_select.map(|hook| {
        format!(
            "if(event.target!==this)return;\
             const index=+this.dataset.index,count=+this.dataset.count;{hook}"
        )
    });
    Ok(view! {
        <div
            role="region"
            aria-roledescription="carousel"
            data-slot="carousel"
            data-orientation=(orientation.as_str())
            data-index="0"
            class=(class!("group/carousel relative", attrs.remove("class")))
            onkeydown=(ARROW_KEYS)
            onpointerenter=(UPDATE)
            onchange=(on_change)
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for the scroll viewport: it scrolls on the carousel's axis, snaps
/// to slides and hides its scrollbar.
const VIEWPORT: StaticClass = class!(
    "overscroll-contain [scrollbar-width:none] [&::-webkit-scrollbar]:hidden \
     group-data-[orientation=horizontal]/carousel:snap-x \
     group-data-[orientation=horizontal]/carousel:overflow-x-auto \
     group-data-[orientation=horizontal]/carousel:overflow-y-hidden \
     group-data-[orientation=vertical]/carousel:snap-y \
     group-data-[orientation=vertical]/carousel:overflow-x-hidden \
     group-data-[orientation=vertical]/carousel:overflow-y-auto snap-mandatory",
);

/// Classes for the track inside the viewport. The negative margin offsets
/// the slides' padding, which spaces them.
const TRACK: StaticClass = class!(
    "flex group-data-[orientation=horizontal]/carousel:-ml-4 \
     group-data-[orientation=vertical]/carousel:-mt-4 \
     group-data-[orientation=vertical]/carousel:flex-col",
);

/// The scrolling area of a [`carousel`]. Pass [`carousel_item`]s as children.
///
/// It renders the viewport and, inside it, the track the slides sit on.
/// `attrs` are forwarded to the track, with extra classes appended (as in
/// Octane): set its height for a vertical carousel, or a smaller negative
/// margin with `!` (`-ml-1!`) to go with smaller slide padding.
#[component]
pub async fn carousel_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="carousel-content"
            class=(VIEWPORT)
            onscroll=(UPDATE)
        >
            <div class=(class!(TRACK, attrs.remove("class"))) (attrs)>
                (child)
            </div>
        </div>
    })
}

/// One slide of a [`carousel`].
///
/// It fills the viewport by default (`basis-full`). Show several at once
/// with a smaller basis, marked important to beat it: `basis-1/2!`, and
/// `lg:basis-1/3!` for each breakpoint. `attrs` are forwarded to the `<div>`,
/// with extra classes appended.
#[component]
pub async fn carousel_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="carousel-item"
            role="group"
            aria-roledescription="slide"
            class=(class!(
                "min-w-0 shrink-0 grow-0 basis-full snap-end \
                 group-data-[orientation=horizontal]/carousel:pl-4 \
                 group-data-[orientation=vertical]/carousel:pt-4",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for the previous button: beside the start of the track, or above
/// it (rotated) when vertical. `size-8!` beats the icon button's `size-10`.
const PREVIOUS: StaticClass = class!(
    "absolute size-8! rounded-base \
     group-data-[orientation=horizontal]/carousel:top-1/2 \
     group-data-[orientation=horizontal]/carousel:-left-12 \
     group-data-[orientation=horizontal]/carousel:-translate-y-1/2 \
     group-data-[orientation=vertical]/carousel:-top-12 \
     group-data-[orientation=vertical]/carousel:left-1/2 \
     group-data-[orientation=vertical]/carousel:-translate-x-1/2 \
     group-data-[orientation=vertical]/carousel:rotate-90",
);

/// Classes for the next button: beside the end of the track, or below it.
const NEXT: StaticClass = class!(
    "absolute h-8! w-8! rounded-base \
     group-data-[orientation=horizontal]/carousel:-right-12 \
     group-data-[orientation=horizontal]/carousel:top-1/2 \
     group-data-[orientation=horizontal]/carousel:-translate-y-1/2 \
     group-data-[orientation=vertical]/carousel:-bottom-12 \
     group-data-[orientation=vertical]/carousel:left-1/2 \
     group-data-[orientation=vertical]/carousel:-translate-x-1/2 \
     group-data-[orientation=vertical]/carousel:rotate-90",
);

/// A button that scrolls the [`carousel`] back by one slide. It starts
/// disabled, at the first slide.
///
/// `variant` defaults to `NoShadow` and `size` to `Icon`. `attrs` are
/// forwarded to the [`button`], with extra classes appended.
#[component]
pub async fn carousel_previous(
    #[default(ButtonVariant::NoShadow)] variant: ButtonVariant,
    #[default(ButtonSize::Icon)] size: ButtonSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        button(
            variant: variant,
            size: size,
            attrs: attributes! {
                type="button"
                data-slot="carousel-previous"
                disabled=""
                onclick=(SCROLL_BY_ONE)
                class=(class!(PREVIOUS, attrs.remove("class")))
                (attrs)
            },
            icon(data: iconify_icon!("lucide:arrow-left"))
            <span class="sr-only">"Previous slide"</span>
        )
    })
}

/// A button that scrolls the [`carousel`] forward by one slide.
///
/// `variant` defaults to `NoShadow` and `size` to `Icon`. `attrs` are
/// forwarded to the [`button`], with extra classes appended.
#[component]
pub async fn carousel_next(
    #[default(ButtonVariant::NoShadow)] variant: ButtonVariant,
    #[default(ButtonSize::Icon)] size: ButtonSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        button(
            variant: variant,
            size: size,
            attrs: attributes! {
                type="button"
                data-slot="carousel-next"
                onclick=(SCROLL_BY_ONE)
                class=(class!(NEXT, attrs.remove("class")))
                (attrs)
            },
            icon(data: iconify_icon!("lucide:arrow-right"))
            <span class="sr-only">"Next slide"</span>
        )
    })
}
