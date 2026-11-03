//! Resizable panels: a flex row (or column) of panels with draggable
//! handles between them, replacing react-resizable-panels.
//!
//! Each [`resizable_panel`] grows by its share of the group (`flex: <size> 1
//! 0px`), so the `default_size`s read as percentages when they add up to 100.
//! A [`resizable_handle`] moves the boundary between the panels on either
//! side of it with a few lines of inline JavaScript: drag it with a pointer,
//! or focus it and press the arrow keys (5% a step), Home or End.
//!
//! Differences from react-resizable-panels: there are no `min_size`,
//! `max_size`, collapsible panels or saved layouts, a handle only resizes its
//! two neighbors, and a handle is told its group's orientation (there is no
//! context to read it from).

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Attributes, Child, View, attributes, class, component, view},
};

/// The direction a [`resizable_panel_group`] lays out its panels in.
///
/// [`Default`] is `ResizableOrientation::Horizontal`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ResizableOrientation {
    /// Panels side by side, resized by dragging left and right.
    #[default]
    Horizontal,
    /// Panels stacked, resized by dragging up and down.
    Vertical,
}

impl ResizableOrientation {
    /// The `aria-orientation` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }

    /// The orientation of a handle in a group of this orientation: a
    /// separator between side-by-side panels is a vertical line.
    fn across(self) -> Self {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}

/// A row or column of [`resizable_panel`]s separated by
/// [`resizable_handle`]s.
///
/// `orientation` defaults to `Horizontal`. Give the group a size (it fills
/// its parent with `h-full w-full`). `attrs` are forwarded to the `<div>`,
/// with extra classes appended.
///
/// ```ignore
/// view! {
///     resizable_panel_group(attrs: attributes! { class="max-w-md rounded-base border-2 border-border" },
///         resizable_panel(default_size: Some(50.0), <div class="h-[200px] p-6">"One"</div>)
///         resizable_handle(with_handle: true)
///         resizable_panel(default_size: Some(50.0), <div class="h-[200px] p-6">"Two"</div>)
///     )
/// }
/// ```
#[component]
pub async fn resizable_panel_group(
    #[default] orientation: ResizableOrientation,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="resizable-panel-group"
            aria-orientation=(orientation.as_str())
            class=(class!(
                "flex h-full w-full font-base aria-[orientation=vertical]:flex-col",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// One panel of a [`resizable_panel_group`].
///
/// `default_size` is its starting share of the group, as a percentage. Give
/// every panel of a group one, or none (they then share the space equally).
/// `attrs` are forwarded to the `<div>`, with extra classes appended; the
/// size is set in its `style`.
#[component]
pub async fn resizable_panel(
    #[default] default_size: Option<f64>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let size = default_size.unwrap_or(1.0).max(0.0);
    Ok(view! {
        <div
            data-slot="resizable-panel"
            style=(format!("flex: {size} 1 0px; overflow: hidden"))
            class=(attrs.remove("class"))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Inline `onpointerdown` for a handle: captures the pointer and records the
/// start position, the pixel sizes of the two neighboring panels and their
/// combined `flex-grow`.
const DRAG_START: &str = "const a=this.previousElementSibling,b=this.nextElementSibling;\
     if(!a||!b||event.button!==0)return;event.preventDefault();this.focus();\
     this.setPointerCapture(event.pointerId);\
     const y=this.getAttribute('aria-orientation')==='horizontal',\
     r=e=>e.getBoundingClientRect()[y?'height':'width'];\
     this._drag={y,x:y?event.clientY:event.clientX,a:r(a),b:r(b),\
     g:(parseFloat(a.style.flexGrow)||0)+(parseFloat(b.style.flexGrow)||0)};\
     this.dataset.dragging=''";

/// Inline `onpointermove` for a handle: splits the two panels' combined
/// size at the pointer, keeping their combined `flex-grow`.
const DRAG_MOVE: &str = "const d=this._drag;if(!d)return;const t=d.a+d.b;if(!t)return;\
     const p=Math.min(t,Math.max(0,d.a+(d.y?event.clientY:event.clientX)-d.x)),\
     ga=d.g*p/t;this.previousElementSibling.style.flexGrow=ga;\
     this.nextElementSibling.style.flexGrow=d.g-ga;\
     this.setAttribute('aria-valuenow',Math.round(100*p/t))";

/// Inline `onpointerup` / `onpointercancel` for a handle: ends the drag.
const DRAG_END: &str = "this._drag=null;delete this.dataset.dragging";

/// Inline `onkeydown` for a handle: the arrow keys along its group move the
/// boundary by 5 (percent, when the sizes add up to 100); Home and End move
/// it all the way.
const KEYS: &str = "const y=this.getAttribute('aria-orientation')==='horizontal',\
     a=this.previousElementSibling,b=this.nextElementSibling;if(!a||!b)return;\
     const ga=parseFloat(a.style.flexGrow)||0,gb=parseFloat(b.style.flexGrow)||0,t=ga+gb,\
     k=event.key,s=k===(y?'ArrowUp':'ArrowLeft')?-5:k===(y?'ArrowDown':'ArrowRight')?5\
     :k==='Home'?-t:k==='End'?t:0;if(!s)return;event.preventDefault();\
     const n=Math.min(t,Math.max(0,ga+s));a.style.flexGrow=n;b.style.flexGrow=t-n;\
     this.setAttribute('aria-valuenow',Math.round(t?100*n/t:0))";

/// A draggable divider between two [`resizable_panel`]s.
///
/// `orientation` is the orientation of the group it sits in (default
/// `Horizontal`); the handle itself runs across it. `with_handle` adds a
/// visible grip. `attrs` are forwarded to the `<div role="separator">`, with
/// extra classes appended.
#[component]
pub async fn resizable_handle(
    #[default] orientation: ResizableOrientation,
    #[default] with_handle: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="resizable-handle"
            role="separator"
            tabindex="0"
            aria-orientation=(orientation.across().as_str())
            aria-valuemin="0"
            aria-valuemax="100"
            onpointerdown=(DRAG_START)
            onpointermove=(DRAG_MOVE)
            onpointerup=(DRAG_END)
            onpointercancel=(DRAG_END)
            onkeydown=(KEYS)
            class=(class!(
                "relative flex w-0.5 items-center justify-center bg-border after:absolute \
                 after:inset-y-0 after:left-1/2 after:w-1 after:-translate-x-1/2 \
                 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-black \
                 focus-visible:ring-offset-1 aria-[orientation=horizontal]:h-0.5 \
                 aria-[orientation=horizontal]:w-full aria-[orientation=horizontal]:after:left-0 \
                 aria-[orientation=horizontal]:after:h-1 aria-[orientation=horizontal]:after:w-full \
                 aria-[orientation=horizontal]:after:-translate-y-1/2 \
                 aria-[orientation=horizontal]:after:translate-x-0 \
                 [&[aria-orientation=horizontal]>div]:rotate-90",
                // Not in Octane, where the library sets the cursor and stops
                // touch scrolling while dragging.
                "cursor-col-resize touch-none select-none \
                 aria-[orientation=horizontal]:cursor-row-resize",
                attrs.remove("class"),
            ))
            (attrs)
        >
            if with_handle {
                <div class="z-10 flex h-4 w-3 items-center justify-center rounded-base border bg-border">
                    icon(data: iconify_icon!("lucide:grip-vertical"), attrs: attributes! { class="size-2.5" })
                </div>
            }
        </div>
    })
}
