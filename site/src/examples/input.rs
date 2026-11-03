use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonVariant, button},
    input::input,
    label::label,
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        input(attrs: attributes! { class="w-[200px]!" type="email" placeholder="Email" })
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        input(attrs: attributes! {
            class="max-w-[200px]" type="email" placeholder="Email" disabled=""
        })
    })
}

#[component]
pub async fn file() -> Result<impl View> {
    Ok(view! {
        <div class="grid w-full max-w-64 items-center gap-1.5">
            label(attrs: attributes! { for="picture" }, "Picture")
            input(attrs: attributes! { id="picture" type="file" })
        </div>
    })
}

#[component]
pub async fn with_button() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-sm items-center space-x-2">
            input(attrs: attributes! { type="email" placeholder="Email" })
            button(variant: ButtonVariant::NoShadow, attrs: attributes! { type="submit" },
                "Subscribe"
            )
        </div>
    })
}

#[component]
pub async fn with_label() -> Result<impl View> {
    Ok(view! {
        <div class="grid max-w-[200px] items-center gap-1.5">
            label(attrs: attributes! { for="email" }, "Email")
            input(attrs: attributes! { type="email" id="email" placeholder="Email" })
        </div>
    })
}
