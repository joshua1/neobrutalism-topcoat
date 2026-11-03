use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    label::label,
    slider::{SliderOrientation, slider},
};

#[component]
pub async fn controlled(cx: &Cx) -> Result<impl View> {
    // A range input reports its value as text, so each thumb's value is
    // kept as the `String` the browser sends.
    let low = signal(cx, || "0.3".to_owned());
    let high = signal(cx, || "0.7".to_owned());

    Ok(view! {
        <div class="grid w-full gap-3">
            <div class="flex items-center justify-between gap-2">
                label(attrs: attributes! { for="slider-demo-temperature" }, "Temperature")
                <span class="text-foreground font-base text-sm">
                    $((low.get(), ", ", high.get()))
                </span>
            </div>
            slider(
                value: vec![0.3, 0.7],
                min: 0.0,
                max: 1.0,
                step: 0.1,
                thumb_attrs: vec![
                    attributes! {
                        id="slider-demo-temperature"
                        @input=$(|e: Event| low.set(e.target.value))
                    },
                    attributes! { @input=$(|e: Event| high.set(e.target.value)) },
                ],
            )
        </div>
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { slider(value: vec![33.0], max: 100.0, step: 1.0) })
}

#[component]
pub async fn two_thumbs() -> Result<impl View> {
    Ok(view! { slider(value: vec![25.0, 50.0], max: 100.0, step: 1.0) })
}

#[component]
pub async fn vertical() -> Result<impl View> {
    Ok(view! {
        slider(
            value: vec![25.0],
            max: 100.0,
            step: 1.0,
            orientation: SliderOrientation::Vertical,
        )
    })
}
