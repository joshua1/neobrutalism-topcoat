use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    progress::{progress, progress_label, progress_value},
    slider::slider,
};

#[component]
pub async fn demo() -> Result<impl View> {
    // Octane sets the value from 13 to 66 on a 500ms timer. Here CSS does it:
    // `@starting-style` (`starting:`) draws the bar at 13% on first render and
    // the indicator's transition grows it to 66% after a 500ms delay.
    // `w-[60%]!` beats the bar's own `w-full`, since `class!` does not drop
    // conflicting classes.
    Ok(view! {
        progress(
            value: 66.0,
            attrs: attributes! {
                aria-label="Progress"
                class="w-[60%]! [&_[data-slot=progress-indicator]]:delay-500 \
                       [&_[data-slot=progress-indicator]]:starting:w-[13%]!"
            },
        )
    })
}

#[component]
pub async fn label() -> Result<impl View> {
    Ok(view! {
        progress(
            value: 56.0,
            attrs: attributes! { class="w-full max-w-sm" aria-labelledby="progress-label" },
            progress_label(attrs: attributes! { id="progress-label" }, "Upload progress")
            progress_value(value: 56.0)
        )
    })
}

#[component]
pub async fn controlled(cx: &Cx) -> Result<impl View> {
    let value = signal(cx, || 50.0);

    Ok(view! {
        <div class="flex w-full max-w-sm flex-col gap-4">
            progress(
                value: $(value.get()),
                attrs: attributes! { aria-label="Progress" class="w-full" },
            )
            slider(
                value: vec![50.0],
                min: 0.0,
                max: 100.0,
                step: 1.0,
                // The runtime renders an `f64` from its own number type, not
                // a bare JavaScript number, so `cx.hydrate` wraps the parsed
                // value; a bare number would leave the bar's width empty.
                thumb_attrs: vec![attributes! {
                    aria-label="Value"
                    @input=$(|e: Event| {
                        let input = e.target.value;
                        value.set(raw!(
                            "cx.hydrate(Number(${input}))",
                            input.to_string().parse().unwrap_or(0.0)
                        ))
                    })
                }],
            )
        </div>
    })
}
