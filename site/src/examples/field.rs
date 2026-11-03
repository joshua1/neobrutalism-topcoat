use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    button::{ButtonVariant, button},
    checkbox::checkbox,
    field::{
        FieldLegendVariant, FieldOrientation, field, field_content, field_description,
        field_error, field_group, field_label, field_legend, field_set, field_title,
    },
    input::input,
    radio_group::{radio_group, radio_group_item},
    switch::switch,
    textarea::textarea,
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn choice_card() -> Result<impl View> {
    let plans = [
        ("free", "Free", "For hobby projects and experiments."),
        ("pro", "Pro", "For professionals and small teams."),
        ("team", "Team", "For growing organizations."),
    ];
    Ok(view! {
        field_set(attrs: attributes! { class="w-full max-w-sm" },
            field_legend(variant: FieldLegendVariant::Label, "Subscription plan")
            radio_group(
                #[key(id)]
                for (id, title, description) in plans {
                    let input_id = format!("plan-{id}");
                    field_label(attrs: attributes! { for=(&input_id) },
                        field(orientation: FieldOrientation::Horizontal,
                            field_content(
                                field_title((title))
                                field_description((description))
                            )
                            radio_group_item(attrs: attributes! {
                                value=(id) id=(&input_id) name="field-plan" checked=(id == "pro")
                            })
                        )
                    )
                }
            )
        )
    })
}

#[component]
pub async fn horizontal() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="max-w-md" },
            field(orientation: FieldOrientation::Horizontal,
                field_content(
                    field_label(attrs: attributes! { for="field-airplane" }, "Airplane Mode")
                    field_description("Turn on airplane mode to disable all connections.")
                )
                switch(attrs: attributes! { id="field-airplane" })
            )
            field(orientation: FieldOrientation::Horizontal,
                checkbox(attrs: attributes! { id="field-newsletter" checked="" })
                field_content(
                    field_label(attrs: attributes! { for="field-newsletter" },
                        "Subscribe to newsletter"
                    )
                    field_description(
                        "Receive weekly updates about new features and promotions."
                    )
                )
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        // Octane's submit handler only called `preventDefault()`.
        <form class="w-full max-w-md" onsubmit="event.preventDefault()">
            field_set(
                field_legend("Contact us")
                field_description("Fill in the form and we will get back to you.")
                field_group(
                    field(
                        field_label(attrs: attributes! { for="field-name" }, "Name")
                        input(attrs: attributes! { id="field-name" placeholder="John Doe" })
                    )
                    field(
                        field_label(attrs: attributes! { for="field-email" }, "Email")
                        input(attrs: attributes! {
                            id="field-email" type="email" placeholder="john@example.com"
                        })
                        field_description("We will never share your email with anyone.")
                    )
                    field(
                        field_label(attrs: attributes! { for="field-message" }, "Message")
                        textarea(attrs: attributes! {
                            id="field-message" placeholder="Your message..."
                        })
                    )
                    field(orientation: FieldOrientation::Horizontal,
                        button(attrs: attributes! { type="submit" }, "Send message")
                        button(variant: ButtonVariant::Neutral, attrs: attributes! { type="button" },
                            "Cancel"
                        )
                    )
                )
            )
        </form>
    })
}

#[component]
pub async fn with_error() -> Result<impl View> {
    Ok(view! {
        field(attrs: attributes! { data-invalid="true" class="max-w-sm" },
            field_label(attrs: attributes! { for="field-username" }, "Username")
            input(attrs: attributes! {
                id="field-username" value="ab" aria-invalid="true" class="border-red-500!"
            })
            field_description("Pick a unique username.")
            field_error(errors: &[
                "Username must be at least 3 characters.",
                "Username is already taken.",
            ])
        )
    })
}
