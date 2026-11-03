//! A panel that slides in from an edge of the screen, with an optional swipe
//! handle, built on the native `<dialog>` element like
//! [`dialog`](super::dialog).
//!
//! The [`drawer`] element is the overlay and viewport (Base UI's
//! `DrawerPortal`, `DrawerOverlay` and `Drawer.Viewport`), and
//! [`drawer_content`] is the sliding popup. Open it with a [`drawer_trigger`]
//! and close it with [`drawer_close`], Escape (modal drawers), or a click on
//! the overlay. No runtime is needed.
//!
//! Not matched from Base UI: swiping to dismiss, snap points, and the scaled
//! stack of nested drawers. The swipe handle is drawn but does not drag.
//! Nested drawers open on top of each other in the top layer.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    runtime::Expr,
    view::{Attributes, Child, StaticClass, View, attributes, class, component, view},
};

use super::dialog::{
    DIALOG_OVERLAY, LIGHT_DISMISS_SCRIPT, SHOW_SCRIPT, dialog_close, dialog_trigger,
};

/// Classes that fade the overlay in and out with the popup's easing. A
/// non-modal drawer has no overlay color (`bg-transparent` beats the
/// `<dialog>` element's default white) and lets clicks through to the page.
/// A drawer nested in another one keeps its overlay clear (`!` beats the
/// modal color), so the page is dimmed once however deep the stack is.
const FADE: StaticClass = class!(
    "min-h-dvh select-none opacity-0 open:opacity-100 starting:open:opacity-0 \
     transition-[opacity,display,overlay] transition-discrete duration-400 open:duration-450 \
     ease-[cubic-bezier(0.32,0.72,0,1)] data-[modal=true]:bg-overlay \
     data-[modal=false]:bg-transparent data-[modal=false]:pointer-events-none \
     in-data-[slot=drawer]:bg-transparent!",
);

/// A drawer: the overlay that holds a [`drawer_content`] popup.
///
/// Give it an `id` and open it with a [`drawer_trigger`] naming that id, or pass
/// a runtime expression to `open` (see [`dialog`](super::dialog::dialog)).
///
/// A non-modal drawer (`modal: false`) has no overlay and leaves the page
/// interactive. Pass the same `modal` to its [`drawer_trigger`], which then
/// opens it with `show()` instead of `showModal()`. Escape does not close a
/// non-modal drawer.
///
/// ```ignore
/// view! {
///     drawer_trigger(
///         target: "move-goal",
///         attrs: attributes! {
///             class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
///         },
///         "Open"
///     )
///     drawer(
///         id: "move-goal",
///         drawer_content(
///             drawer_header(
///                 drawer_title("Move goal")
///                 drawer_description("Set your daily activity goal.")
///             )
///             drawer_footer(drawer_close("Close"))
///         )
///     )
/// }
/// ```
#[component]
pub async fn drawer(
    /// Whether the drawer shows without being opened by a trigger.
    #[into]
    #[default(Expr::from(false))]
    open: Expr<bool>,
    /// The `id` that [`drawer_trigger`]'s `target` refers to.
    #[into]
    #[default]
    id: Option<&str>,
    /// Whether the drawer covers the page with an overlay. Must match the
    /// trigger's `modal`.
    #[default(true)]
    modal: bool,
    /// Keeps a click on the overlay from closing the drawer.
    #[default]
    disable_pointer_dismissal: bool,
    /// Extra attributes for the `<dialog>` element.
    #[default]
    mut attrs: Attributes,
    /// The drawer's content, usually a [`drawer_content`].
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let dismiss = (modal && !disable_pointer_dismissal).then_some(LIGHT_DISMISS_SCRIPT);

    Ok(view! {
        <dialog
            data-slot="drawer"
            data-modal=(if modal { "true" } else { "false" })
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

/// A button that opens the [`drawer`] whose `id` is `target`.
///
/// Set `modal: false` for a non-modal drawer. It is unstyled; give it button
/// styling with `class=(button_variants(..))`.
#[component]
pub async fn drawer_trigger(
    /// The `id` of the drawer to open.
    target: &str,
    /// Whether to open the drawer as a modal. Must match the drawer's `modal`.
    #[default(true)]
    modal: bool,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        if modal {
            dialog_trigger(
                target: target,
                attrs: attributes! { data-slot="drawer-trigger" (attrs) },
                (child)
            )
        } else {
            <button
                type="button"
                data-slot="drawer-trigger"
                aria-haspopup="dialog"
                aria-controls=(target)
                onclick=(SHOW_SCRIPT)
                (attrs)
            >
                (child)
            </button>
        }
    })
}

/// A button that closes the [`drawer`] it is inside. It is unstyled.
#[component]
pub async fn drawer_close(
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        dialog_close(attrs: attributes! { data-slot="drawer-close" (attrs) }, (child))
    })
}

/// The edge a [`drawer_content`] slides in from, named after the direction a
/// swipe would dismiss it.
///
/// [`Default`] is `DrawerSwipeDirection::Down`, a bottom drawer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DrawerSwipeDirection {
    /// From the top edge.
    Up,
    /// From the bottom edge.
    #[default]
    Down,
    /// From the left edge.
    Left,
    /// From the right edge.
    Right,
}

impl DrawerSwipeDirection {
    /// The value of the popup's `data-swipe-direction` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// The value of the popup's `data-swipe-axis` attribute: `y` for drawers
    /// at the top or bottom, `x` for drawers at the sides.
    fn axis(self) -> &'static str {
        if matches!(self, Self::Up | Self::Down) {
            "y"
        } else {
            "x"
        }
    }
}

/// A grab bar drawn at the edge of a [`drawer_content`]. It is decorative and
/// does not drag.
#[component]
pub async fn drawer_swipe_handle(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="drawer-swipe-handle"
            aria-hidden="true"
            class=(class!(
                "relative z-10 flex shrink-0 cursor-grab transition-opacity duration-200 \
                 group-data-[swipe-axis=x]/drawer-popup:h-full \
                 group-data-[swipe-axis=x]/drawer-popup:w-4 \
                 group-data-[swipe-axis=x]/drawer-popup:items-center \
                 group-data-[swipe-axis=y]/drawer-popup:h-4 \
                 group-data-[swipe-axis=y]/drawer-popup:w-full \
                 group-data-[swipe-axis=y]/drawer-popup:justify-center \
                 group-data-[swipe-direction=down]/drawer-popup:items-end \
                 group-data-[swipe-direction=left]/drawer-popup:order-last \
                 group-data-[swipe-direction=left]/drawer-popup:justify-start \
                 group-data-[swipe-direction=right]/drawer-popup:justify-end \
                 group-data-[swipe-direction=up]/drawer-popup:order-last \
                 group-data-[swipe-direction=up]/drawer-popup:items-start \
                 after:block after:shrink-0 after:rounded-full after:bg-current \
                 group-data-[swipe-axis=x]/drawer-popup:after:h-[100px] \
                 group-data-[swipe-axis=x]/drawer-popup:after:w-2 \
                 group-data-[swipe-axis=y]/drawer-popup:after:h-2 \
                 group-data-[swipe-axis=y]/drawer-popup:after:w-[100px] active:cursor-grabbing",
                attrs.remove("class"),
            ))
            (attrs)
        ></div>
    })
}

/// Classes for the popup: its edge, border, size and slide.
///
/// Every direction rule keys off the popup's own `data-swipe-direction` and
/// `data-swipe-axis`, so descendants can use the `group/drawer-popup` group
/// too (`group-data-[swipe-axis=x]/drawer-popup:size-full`).
const POPUP: StaticClass = class!(
    // Base.
    "group/drawer-popup pointer-events-auto fixed z-50 m-(--drawer-inset,0px) flex \
     h-(--drawer-content-height) max-h-(--drawer-content-max-height,none) min-h-0 \
     w-(--drawer-content-width,auto) flex-col bg-background text-foreground border-border \
     outline-none select-none [interpolate-size:allow-keywords] \
     data-[swipe-direction=down]:rounded-t-base data-[swipe-direction=down]:border-t-2 \
     data-[swipe-direction=left]:rounded-r-base data-[swipe-direction=left]:border-r-2 \
     data-[swipe-direction=right]:rounded-l-base data-[swipe-direction=right]:border-l-2 \
     data-[swipe-direction=up]:rounded-b-base data-[swipe-direction=up]:border-b-2 \
     after:pointer-events-none after:absolute \
     after:bg-(--drawer-bleed-background,var(--color-background)) \
     data-[swipe-axis=x]:after:inset-y-0 data-[swipe-axis=x]:after:w-(--bleed) \
     data-[swipe-axis=y]:after:inset-x-0 data-[swipe-axis=y]:after:h-(--bleed) \
     data-[swipe-direction=down]:after:top-full data-[swipe-direction=left]:after:right-full \
     data-[swipe-direction=right]:after:left-full data-[swipe-direction=up]:after:bottom-full \
     [--bleed:3rem] \
     [--drawer-content-height:var(--drawer-height,auto)] \
     data-[swipe-axis=x]:[--drawer-content-width:75%] \
     data-[swipe-axis=y]:[--drawer-content-max-height:calc(100dvh-6rem)] \
     data-[swipe-axis=x]:sm:[--drawer-content-width:24rem] \
     transform-(--closed-transform) group-open/dialog:transform-none \
     starting:group-open/dialog:transform-(--closed-transform) \
     transition-[transform,height,opacity,filter] duration-400 group-open/dialog:duration-450 \
     ease-[cubic-bezier(0.22,1,0.36,1)] will-change-transform \
     data-[swipe-axis=y]:inset-x-0 data-[swipe-axis=x]:inset-y-0 data-[swipe-axis=x]:flex-row \
     data-[swipe-direction=down]:bottom-0 data-[swipe-direction=down]:origin-bottom \
     data-[swipe-direction=down]:[--closed-transform:translate3d(0,calc(100%+var(--drawer-inset,0px)+2px),0)] \
     data-[swipe-direction=up]:top-0 data-[swipe-direction=up]:origin-top \
     data-[swipe-direction=up]:[--closed-transform:translate3d(0,calc(-100%-var(--drawer-inset,0px)-2px),0)] \
     data-[swipe-direction=left]:left-0 data-[swipe-direction=left]:origin-left \
     data-[swipe-direction=left]:[--closed-transform:translate3d(calc(-100%-var(--drawer-inset,0px)-2px),0,0)] \
     data-[swipe-direction=right]:right-0 data-[swipe-direction=right]:origin-right \
     data-[swipe-direction=right]:[--closed-transform:translate3d(calc(100%+var(--drawer-inset,0px)+2px),0,0)]",
);

/// The sliding popup inside a [`drawer`], holding the drawer's sections.
///
/// `swipe_direction` picks the edge (a bottom drawer by default). The swipe
/// handle shows for bottom drawers unless `show_swipe_handle` says otherwise.
/// `attrs` go on the popup, with extra classes appended; the children go in a
/// scroll-contained column inside it.
#[component]
pub async fn drawer_content(
    /// The edge the drawer slides in from.
    #[default]
    swipe_direction: DrawerSwipeDirection,
    /// Whether to draw the swipe handle. Defaults to `true` for `Down` only.
    #[default]
    show_swipe_handle: Option<bool>,
    /// Extra attributes for the popup `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The drawer's sections.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let show_swipe_handle =
        show_swipe_handle.unwrap_or(swipe_direction == DrawerSwipeDirection::Down);

    Ok(view! {
        <div
            data-slot="drawer-popup"
            data-swipe-direction=(swipe_direction.as_str())
            data-swipe-axis=(swipe_direction.axis())
            class=(class!(POPUP, attrs.remove("class")))
            (attrs)
        >
            if show_swipe_handle {
                drawer_swipe_handle()
            }
            <div
                data-slot="drawer-content"
                class="flex min-h-0 flex-1 flex-col overflow-hidden overscroll-contain \
                    rounded-[inherit] transition-opacity duration-300 \
                    ease-[cubic-bezier(0.45,1.005,0,1.005)] select-text"
            >
                (child)
            </div>
        </div>
    })
}

/// The opening section of a [`drawer_content`], stacking a [`drawer_title`]
/// and a [`drawer_description`].
#[component]
pub async fn drawer_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="drawer-header"
            class=(class!("grid gap-1.5 p-4 text-center sm:text-left", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The actions at the bottom of a [`drawer_content`], stacked.
#[component]
pub async fn drawer_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="drawer-footer"
            class=(class!("mt-auto flex flex-col gap-3 p-4", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`drawer`], rendered as an `<h2>`.
#[component]
pub async fn drawer_title(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2
            data-slot="drawer-title"
            class=(class!("text-lg font-heading leading-none tracking-tight", attrs.remove("class")))
            (attrs)
        >
            (child)
        </h2>
    })
}

/// Text that explains the drawer's purpose, rendered as a `<p>`.
#[component]
pub async fn drawer_description(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <p
            data-slot="drawer-description"
            class=(class!("text-sm font-base text-foreground", attrs.remove("class")))
            (attrs)
        >
            (child)
        </p>
    })
}
