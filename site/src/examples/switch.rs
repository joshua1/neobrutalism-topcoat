use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    field::{
        FieldOrientation, field, field_content, field_description, field_group, field_label,
        field_title,
    },
    label::label,
    switch::{SwitchSize, switch},
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn choice_card() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="w-full max-w-sm" },
            field_label(attrs: attributes! { for="switch-share" },
                field(orientation: FieldOrientation::Horizontal,
                    field_content(
                        field_title("Share across devices")
                        field_description(
                            "Focus is shared across devices, and turns off when you leave the app."
                        )
                    )
                    switch(attrs: attributes! { id="switch-share" })
                )
            )
            field_label(attrs: attributes! { for="switch-notifications" },
                field(orientation: FieldOrientation::Horizontal,
                    field_content(
                        field_title("Enable notifications")
                        field_description(
                            "Receive notifications when focus mode is enabled or disabled."
                        )
                    )
                    switch(attrs: attributes! { id="switch-notifications" checked="" })
                )
            )
        )
    })
}

#[component]
pub async fn description() -> Result<impl View> {
    Ok(view! {
        field(orientation: FieldOrientation::Horizontal, attrs: attributes! { class="max-w-sm" },
            field_content(
                field_label(attrs: attributes! { for="switch-focus-mode" }, "Share across devices")
                field_description(
                    "Focus is shared across devices, and turns off when you leave the app."
                )
            )
            switch(attrs: attributes! { id="switch-focus-mode" })
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        field(
            orientation: FieldOrientation::Horizontal,
            attrs: attributes! { data-disabled="true" class="w-fit!" },
            switch(attrs: attributes! { id="switch-disabled-unchecked" disabled="" })
            field_label(attrs: attributes! { for="switch-disabled-unchecked" }, "Disabled")
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center space-x-2">
            switch(attrs: attributes! { id="airplane-mode" })
            label(attrs: attributes! { for="airplane-mode" }, "Airplane Mode")
        </div>
    })
}

#[component]
pub async fn invalid() -> Result<impl View> {
    Ok(view! {
        field(
            orientation: FieldOrientation::Horizontal,
            attrs: attributes! { class="max-w-sm" data-invalid="true" },
            field_content(
                field_label(attrs: attributes! { for="switch-terms" }, "Accept terms and conditions")
                field_description("You must accept the terms and conditions to continue.")
            )
            switch(attrs: attributes! { id="switch-terms" aria-invalid="true" })
        )
    })
}

#[component]
pub async fn size() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="w-full max-w-[10rem]" },
            field(orientation: FieldOrientation::Horizontal,
                switch(size: SwitchSize::Sm, attrs: attributes! { id="switch-size-sm" })
                field_label(attrs: attributes! { for="switch-size-sm" }, "Small")
            )
            field(orientation: FieldOrientation::Horizontal,
                switch(size: SwitchSize::Default, attrs: attributes! { id="switch-size-default" })
                field_label(attrs: attributes! { for="switch-size-default" }, "Default")
            )
        )
    })
}
