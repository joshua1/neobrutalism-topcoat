// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use std::iter;

use topcoat::{
    Result,
    view::{Attributes, StaticClass, View, class, component, view},
};

/// The direction a [`slider`] runs in.
///
/// [`Default`] is `SliderOrientation::Horizontal`, used when no orientation
/// is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SliderOrientation {
    /// Left (minimum) to right (maximum).
    #[default]
    Horizontal,
    /// Bottom (minimum) to top (maximum).
    Vertical,
}

impl SliderOrientation {
    /// The `data-orientation` value, which every part's classes read.
    fn as_str(self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Classes for the area holding the track and thumbs.
///
/// Base UI's `data-disabled:` becomes `has-disabled:`, read from the inputs.
const CONTROL: StaticClass = class!(
    "relative flex w-full touch-none select-none items-center has-disabled:opacity-50 \
     data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 \
     data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col",
);

/// Classes for the bordered track.
const TRACK: StaticClass = class!(
    "relative grow overflow-hidden rounded-base bg-secondary-background border-2 border-border \
     select-none data-[orientation=horizontal]:h-3 data-[orientation=horizontal]:w-full \
     data-[orientation=vertical]:h-full data-[orientation=vertical]:w-3",
);

/// Classes for the filled part of the track. Its position is an inline
/// style, as with Base UI.
const RANGE: StaticClass = class!(
    "bg-main select-none data-[orientation=horizontal]:h-full \
     data-[orientation=vertical]:w-full",
);

/// Classes laying each native range input over the track, one thumb tall
/// (or wide, when vertical), with its own track made invisible. A vertical
/// input is turned with `writing-mode`, minimum at the bottom. It needs an
/// explicit `h-full`: `inset-y-0` alone leaves a range input at its intrinsic
/// length, shorter than the track.
const THUMB_INPUT: StaticClass = class!(
    "absolute m-0 cursor-pointer appearance-none bg-transparent outline-none \
     disabled:cursor-not-allowed \
     data-[orientation=horizontal]:inset-x-0 data-[orientation=horizontal]:top-1/2 \
     data-[orientation=horizontal]:h-5 data-[orientation=horizontal]:-translate-y-1/2 \
     data-[orientation=vertical]:inset-y-0 data-[orientation=vertical]:h-full \
     data-[orientation=vertical]:left-1/2 \
     data-[orientation=vertical]:w-5 data-[orientation=vertical]:-translate-x-1/2 \
     data-[orientation=vertical]:[writing-mode:vertical-lr] \
     data-[orientation=vertical]:[direction:rtl] \
     [&::-webkit-slider-runnable-track]:bg-transparent [&::-moz-range-track]:bg-transparent",
);

/// Octane's thumb classes on the WebKit/Blink thumb pseudo-element.
///
/// `has-focus-visible:` and `data-disabled:` (read from Base UI's hidden
/// input) become `focus-visible:` and `disabled:` on the input itself.
const WEBKIT_THUMB: StaticClass = class!(
    "[&::-webkit-slider-thumb]:pointer-events-auto [&::-webkit-slider-thumb]:block \
     [&::-webkit-slider-thumb]:size-5 [&::-webkit-slider-thumb]:shrink-0 \
     [&::-webkit-slider-thumb]:appearance-none [&::-webkit-slider-thumb]:rounded-full \
     [&::-webkit-slider-thumb]:border-2 [&::-webkit-slider-thumb]:border-border \
     [&::-webkit-slider-thumb]:bg-white [&::-webkit-slider-thumb]:ring-offset-white \
     [&::-webkit-slider-thumb]:transition-colors [&::-webkit-slider-thumb]:select-none \
     focus-visible:[&::-webkit-slider-thumb]:outline-none \
     focus-visible:[&::-webkit-slider-thumb]:ring-2 \
     focus-visible:[&::-webkit-slider-thumb]:ring-ring \
     focus-visible:[&::-webkit-slider-thumb]:ring-offset-2 \
     disabled:[&::-webkit-slider-thumb]:pointer-events-none \
     disabled:[&::-webkit-slider-thumb]:opacity-50",
);

/// Octane's thumb classes on the Firefox thumb pseudo-element.
const MOZ_THUMB: StaticClass = class!(
    "[&::-moz-range-thumb]:pointer-events-auto [&::-moz-range-thumb]:block \
     [&::-moz-range-thumb]:size-5 [&::-moz-range-thumb]:shrink-0 \
     [&::-moz-range-thumb]:box-border [&::-moz-range-thumb]:rounded-full \
     [&::-moz-range-thumb]:border-2 [&::-moz-range-thumb]:border-border \
     [&::-moz-range-thumb]:bg-white [&::-moz-range-thumb]:ring-offset-white \
     [&::-moz-range-thumb]:transition-colors [&::-moz-range-thumb]:select-none \
     focus-visible:[&::-moz-range-thumb]:outline-none \
     focus-visible:[&::-moz-range-thumb]:ring-2 \
     focus-visible:[&::-moz-range-thumb]:ring-ring \
     focus-visible:[&::-moz-range-thumb]:ring-offset-2 \
     disabled:[&::-moz-range-thumb]:pointer-events-none \
     disabled:[&::-moz-range-thumb]:opacity-50",
);

/// Keeps the thumbs in order and the filled range between them, on every
/// `input` event that bubbles up from a thumb.
///
/// A thumb dragged past its neighbour is pushed back, then a fresh `input`
/// event is sent so the caller's `@input` handler sees the corrected value.
const SYNC: &str = "const s = [...this.querySelectorAll('input')], t = event.target, \
     i = s.indexOf(t); if (i < 0) return; \
     const lo = i > 0 ? +s[i - 1].value : -Infinity, \
     hi = i < s.length - 1 ? +s[i + 1].value : Infinity, \
     v = Math.min(Math.max(+t.value, lo), hi); \
     if (v != +t.value) { t.value = v; t.dispatchEvent(new Event('input', { bubbles: true })); \
     return; } \
     const p = s.map(e => (e.value - e.min) / (e.max - e.min) * 100), \
     a = s.length > 1 ? p[0] : 0, b = p[p.length - 1], \
     r = this.querySelector('[data-slot=slider-range]').style; \
     if (this.dataset.orientation == 'vertical') { r.bottom = a + '%'; r.height = b - a + '%'; } \
     else { r.left = a + '%'; r.width = b - a + '%'; }";

/// A neobrutalism slider built on native `<input type="range">` thumbs.
///
/// `value` holds one number per thumb (Octane's `defaultValue`); left empty
/// it is `[min, max]`, two thumbs, as in Octane. `min`, `max` and `step`
/// default to 0, 100 and 1. Pass `disabled` to disable every thumb.
///
/// `attrs` go on the root `<div>`, with extra classes appended. Per-thumb
/// attributes go in `thumb_attrs`, matched to `value` by position: `id`,
/// `name`, `aria-label`, and `@input` / `@change` handlers reading
/// `e.target.value` (a string).
///
/// The browser owns the values; a small inline script keeps the thumbs from
/// crossing and redraws the filled range. With two thumbs, only the thumbs
/// take pointer input (clicking the bare track does nothing), and when they
/// overlap at the maximum the upper one must be moved first. Vertical
/// sliders rely on `writing-mode` support for range inputs.
///
/// ```ignore
/// view! {
///     slider(value: vec![33.0], attrs: attributes! { class="w-[60%]" })
/// }
/// ```
#[component]
pub async fn slider(
    #[default] value: Vec<f64>,
    #[default(0.0)] min: f64,
    #[default(100.0)] max: f64,
    #[default(1.0)] step: f64,
    #[default] orientation: SliderOrientation,
    #[default] disabled: bool,
    #[default] thumb_attrs: Vec<Attributes>,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let value = if value.is_empty() { vec![min, max] } else { value };

    // Where a value sits along the track, from 0 to 100.
    let percent = |v: f64| {
        if max > min {
            ((v - min) / (max - min) * 100.0).clamp(0.0, 100.0)
        } else {
            0.0
        }
    };
    let start = if value.len() > 1 { percent(value[0]) } else { 0.0 };
    let end = value.last().copied().map_or(0.0, percent);
    let range_style = match orientation {
        SliderOrientation::Horizontal => {
            format!("position:absolute;left:{start}%;width:{}%", end - start)
        }
        SliderOrientation::Vertical => {
            format!("position:absolute;bottom:{start}%;height:{}%", end - start)
        }
    };

    // Overlapping inputs would swallow each other's clicks, so with several
    // thumbs only the thumbs themselves take pointer input.
    let several = value.len() > 1;
    let thumbs: Vec<(String, Attributes)> = value
        .iter()
        .map(|v| v.to_string())
        .zip(thumb_attrs.into_iter().chain(iter::repeat_with(Attributes::new)))
        .collect();

    let orientation = orientation.as_str();
    let (min, max, step) = (min.to_string(), max.to_string(), step.to_string());

    Ok(view! {
        <div
            data-slot="slider"
            data-orientation=(orientation)
            class=(class!(
                "data-[orientation=horizontal]:w-full data-[orientation=vertical]:h-full",
                attrs.remove("class"),
            ))
            (attrs)
        >
            <div
                data-slot="slider-control"
                data-orientation=(orientation)
                class=(CONTROL)
                oninput=(SYNC)
            >
                <div data-slot="slider-track" data-orientation=(orientation) class=(TRACK)>
                    <div
                        data-slot="slider-range"
                        data-orientation=(orientation)
                        class=(RANGE)
                        style=(range_style)
                    ></div>
                </div>
                for (thumb_value, mut thumb) in thumbs {
                    <input
                        type="range"
                        data-slot="slider-thumb"
                        data-orientation=(orientation)
                        aria-orientation=(orientation)
                        min=(&min)
                        max=(&max)
                        step=(&step)
                        value=(thumb_value)
                        disabled=(disabled)
                        class=(class!(
                            THUMB_INPUT,
                            WEBKIT_THUMB,
                            MOZ_THUMB,
                            "pointer-events-none" if several,
                            thumb.remove("class"),
                        ))
                        (thumb)
                    >
                }
            </div>
        </div>
    })
}
