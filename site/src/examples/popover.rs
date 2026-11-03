//! The Octane examples use `Input`, `Label` and `Field`, which are ported in
//! another batch; they are plain elements with the same classes here.

use topcoat::{
    Result,
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button_variants};
use crate::components::popover::{
    PopoverAlign, popover, popover_content, popover_description, popover_header, popover_title,
    popover_trigger,
};

/// `Label` classes, from `label.tsrx`.
const LABEL: &str = "text-sm font-heading leading-none peer-disabled:cursor-not-allowed \
    peer-disabled:opacity-70";

/// `Input` classes, from `input.tsrx`.
const INPUT: &str = "flex h-10 w-full rounded-base border-2 border-border bg-secondary-background \
    selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base text-foreground \
    placeholder:text-foreground/50 focus-visible:outline-hidden focus-visible:ring-2 \
    focus-visible:ring-black focus-visible:ring-offset-2 disabled:cursor-not-allowed \
    disabled:opacity-50";

#[component]
pub async fn demo() -> Result<impl View> {
    // The `!` on `w-80` and `h-8` overrides the panel's `w-72` and the input's
    // `h-10`.
    Ok(view! {
        popover(
            popover_trigger(
                target: "popover-demo",
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Open popover"
            )
            popover_content(id: "popover-demo", attrs: attributes! { class="w-80!" },
                <div class="grid gap-4">
                    <div class="space-y-2">
                        <h4 class="font-heading leading-none">"Dimensions"</h4>
                        <p class="text-sm">"Set the dimensions for the layer."</p>
                    </div>
                    <div class="grid gap-2">
                        for (id, label, value) in [
                            ("width", "Width", "100%"),
                            ("maxWidth", "Max. width", "300px"),
                            ("height", "Height", "25px"),
                            ("maxHeight", "Max. height", "none"),
                        ] {
                            <div class="grid grid-cols-3 items-center gap-4">
                                <label for=(id) class=(LABEL)>(label)</label>
                                <input
                                    id=(id)
                                    value=(value)
                                    class=(class!(INPUT, "col-span-2 h-8!"))
                                >
                            </div>
                        }
                    </div>
                </div>
            )
        )
    })
}

#[component]
pub async fn align() -> Result<impl View> {
    // `w-40!` overrides the panel's `w-72`.
    Ok(view! {
        <div class="flex gap-6">
            for (id, label, align, text) in [
                ("popover-align-start", "Start", PopoverAlign::Start, "Aligned to start"),
                ("popover-align-center", "Center", PopoverAlign::Center, "Aligned to center"),
                ("popover-align-end", "End", PopoverAlign::End, "Aligned to end"),
            ] {
                popover(
                    popover_trigger(
                        target: id,
                        attrs: attributes! {
                            class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Sm))
                        },
                        (label)
                    )
                    popover_content(id: id, align: align, attrs: attributes! { class="w-40!" },
                        (text)
                    )
                )
            }
        </div>
    })
}

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        popover(
            popover_trigger(
                target: "popover-basic",
                attrs: attributes! {
                    class=(class!(
                        button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                        "w-fit",
                    ))
                },
                "Open Popover"
            )
            popover_content(id: "popover-basic", align: PopoverAlign::Start,
                popover_header(
                    popover_title("Dimensions")
                    popover_description("Set the dimensions for the layer.")
                )
            )
        )
    })
}

#[component]
pub async fn with_form() -> Result<impl View> {
    // `w-64!` overrides the panel's `w-72`. The rows are `Field`s with
    // `orientation="horizontal"` in Octane.
    Ok(view! {
        popover(
            popover_trigger(
                target: "popover-with-form",
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::NoShadow, ButtonSize::Default))
                },
                "Open Popover"
            )
            popover_content(
                id: "popover-with-form",
                align: PopoverAlign::Start,
                attrs: attributes! { class="w-64!" },
                popover_header(
                    popover_title("Dimensions")
                    popover_description("Set the dimensions for the layer.")
                )
                <div class="group/field-group flex w-full flex-col gap-4">
                    for (id, label, value) in [
                        ("popover-form-width", "Width", "100%"),
                        ("popover-form-height", "Height", "25px"),
                    ] {
                        <div class="group/field flex w-full flex-row items-center gap-2 font-base">
                            <label for=(id) class=(class!(LABEL, "w-1/2"))>(label)</label>
                            <input id=(id) value=(value) class=(INPUT)>
                        </div>
                    }
                </div>
            )
        )
    })
}
