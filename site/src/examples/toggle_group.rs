use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::toggle::{ToggleKind, ToggleSize, ToggleVariant};
use crate::components::toggle_group::{ToggleGroupOrientation, toggle_group, toggle_group_item};

/// Several can be pressed at once: independent (checkbox) items.
#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        toggle_group(
            toggle_group_item(attrs: attributes! { name="bold" aria-label="Toggle bold" checked="" },
                icon(data: iconify_icon!("lucide:bold"))
            )
            toggle_group_item(attrs: attributes! { name="italic" aria-label="Toggle italic" },
                icon(data: iconify_icon!("lucide:italic"))
            )
            toggle_group_item(attrs: attributes! { name="underline" aria-label="Toggle underline" },
                icon(data: iconify_icon!("lucide:underline"))
            )
        )
    })
}

/// The Octane version stores the weight in `useState`; here a runtime signal
/// follows the checked radio button and updates the hint in the browser.
///
/// Octane's `Field`, `FieldLabel` and `FieldDescription` belong to another
/// component, so plain elements stand in for them.
#[component]
pub async fn custom(cx: &Cx) -> Result<impl View> {
    let weight = signal(cx, || String::from("normal"));
    let weights = [
        ("light", "Light", "font-light"),
        ("normal", "Normal", "font-normal"),
        ("medium", "Medium", "font-medium"),
        ("bold", "Bold", "font-bold"),
    ];

    Ok(view! {
        <div role="group" class="flex w-full max-w-xs flex-col gap-3">
            <div class="text-sm font-heading leading-none">"Font Weight"</div>
            toggle_group(
                #[key(value)]
                for (value, label, font) in weights {
                    toggle_group_item(
                        kind: ToggleKind::Exclusive,
                        variant: ToggleVariant::Outline,
                        size: ToggleSize::Lg,
                        attrs: attributes! {
                            // `size-16` overrides the large size's `h-11 min-w-11`.
                            class="flex size-16! flex-col items-center justify-center rounded-base"
                            name="font_weight"
                            value=(value)
                            aria-label=(label)
                            checked=(value == "normal")
                            @change=$(|e: Event| weight.set(e.target.value))
                        },
                        <span class=(format!("text-2xl leading-none {font}"))>"Aa"</span>
                        <span class="text-xs text-foreground">(label)</span>
                    )
                }
            )
            <p class="text-sm font-base">
                "Use "
                <code class="rounded-base bg-secondary-background px-1 py-0.5 font-mono">
                    "font-" $(weight.get())
                </code>
                " to set the font weight."
            </p>
        </div>
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        toggle_group(attrs: attributes! { disabled="" },
            toggle_group_item(attrs: attributes! { name="bold" aria-label="Toggle bold" },
                icon(data: iconify_icon!("lucide:bold"))
            )
            toggle_group_item(attrs: attributes! { name="italic" aria-label="Toggle italic" },
                icon(data: iconify_icon!("lucide:italic"))
            )
            toggle_group_item(
                attrs: attributes! { name="strikethrough" aria-label="Toggle strikethrough" },
                icon(data: iconify_icon!("lucide:underline"))
            )
        )
    })
}

#[component]
pub async fn outline() -> Result<impl View> {
    Ok(view! {
        toggle_group(
            toggle_group_item(
                kind: ToggleKind::Exclusive,
                variant: ToggleVariant::Outline,
                attrs: attributes! { name="outline" value="all" aria-label="Toggle all" checked="" },
                "All"
            )
            toggle_group_item(
                kind: ToggleKind::Exclusive,
                variant: ToggleVariant::Outline,
                attrs: attributes! { name="outline" value="missed" aria-label="Toggle missed" },
                "Missed"
            )
        )
    })
}

#[component]
pub async fn single() -> Result<impl View> {
    let items = [("all", "All"), ("missed", "Missed"), ("archived", "Archived")];

    Ok(view! {
        toggle_group(
            #[key(value)]
            for (value, label) in items {
                toggle_group_item(
                    kind: ToggleKind::Exclusive,
                    attrs: attributes! {
                        name="single"
                        value=(value)
                        aria-label=(format!("Show {}", label.to_lowercase()))
                        checked=(value == "all")
                    },
                    (label)
                )
            }
        )
    })
}

#[component]
pub async fn size() -> Result<impl View> {
    let sides = [("top", "Top"), ("bottom", "Bottom"), ("left", "Left"), ("right", "Right")];

    Ok(view! {
        <div class="flex flex-col gap-4">
            #[key(size_name)]
            for (size, size_name) in [(ToggleSize::Sm, "size_sm"), (ToggleSize::Default, "size_default")] {
                toggle_group(
                    #[key(value)]
                    for (value, label) in sides {
                        toggle_group_item(
                            kind: ToggleKind::Exclusive,
                            variant: ToggleVariant::Outline,
                            size: size,
                            attrs: attributes! {
                                name=(size_name)
                                value=(value)
                                aria-label=(format!("Toggle {value}"))
                                checked=(value == "top")
                            },
                            (label)
                        )
                    }
                )
            }
        </div>
    })
}

#[component]
pub async fn spacing() -> Result<impl View> {
    Ok(view! {
        toggle_group(spacing: 0,
            toggle_group_item(
                kind: ToggleKind::Exclusive,
                attrs: attributes! { name="align" value="left" aria-label="Align left" checked="" },
                icon(data: iconify_icon!("lucide:align-left"))
            )
            toggle_group_item(
                kind: ToggleKind::Exclusive,
                attrs: attributes! { name="align" value="center" aria-label="Align center" },
                icon(data: iconify_icon!("lucide:align-center"))
            )
            toggle_group_item(
                kind: ToggleKind::Exclusive,
                attrs: attributes! { name="align" value="right" aria-label="Align right" },
                icon(data: iconify_icon!("lucide:align-right"))
            )
        )
    })
}

#[component]
pub async fn vertical() -> Result<impl View> {
    Ok(view! {
        toggle_group(orientation: ToggleGroupOrientation::Vertical,
            toggle_group_item(attrs: attributes! { name="bold" aria-label="Toggle bold" },
                icon(data: iconify_icon!("lucide:bold"))
            )
            toggle_group_item(
                attrs: attributes! { name="italic" aria-label="Toggle italic" checked="" },
                icon(data: iconify_icon!("lucide:italic"))
            )
            toggle_group_item(attrs: attributes! { name="underline" aria-label="Toggle underline" },
                icon(data: iconify_icon!("lucide:underline"))
            )
        )
    })
}
