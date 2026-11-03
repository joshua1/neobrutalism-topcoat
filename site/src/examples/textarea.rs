use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    field::{field_description, field_label},
    textarea::textarea,
};

// The `button` and `field` examples share their names with the components
// they use, so those components are called by their full paths.

#[component]
pub async fn button() -> Result<impl View> {
    Ok(view! {
        <div class="grid w-full max-w-xs gap-2">
            textarea(attrs: attributes! { placeholder="Type your message here." })
            crate::components::button::button("Send message")
        </div>
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        textarea(attrs: attributes! { placeholder="Type your message here." })
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        crate::components::field::field(
            attrs: attributes! { data-disabled="true" class="max-w-xs" },
            field_label(attrs: attributes! { for="textarea-disabled" }, "Message")
            textarea(attrs: attributes! {
                id="textarea-disabled" placeholder="Type your message here." disabled=""
            })
        )
    })
}

#[component]
pub async fn field() -> Result<impl View> {
    Ok(view! {
        crate::components::field::field(attrs: attributes! { class="max-w-xs" },
            field_label(attrs: attributes! { for="textarea-message" }, "Message")
            field_description("Enter your message below.")
            textarea(attrs: attributes! {
                id="textarea-message" placeholder="Type your message here."
            })
        )
    })
}

#[component]
pub async fn invalid() -> Result<impl View> {
    Ok(view! {
        crate::components::field::field(
            attrs: attributes! { data-invalid="true" class="max-w-xs" },
            field_label(attrs: attributes! { for="textarea-invalid" }, "Message")
            textarea(attrs: attributes! {
                id="textarea-invalid" placeholder="Type your message here." aria-invalid="true"
            })
            field_description("Please enter a valid message.")
        )
    })
}
