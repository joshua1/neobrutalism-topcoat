use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    field::{
        FieldLegendVariant, FieldOrientation, field, field_content, field_description,
        field_label, field_legend, field_set, field_title,
    },
    label::label,
    radio_group::{radio_group, radio_group_item},
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

// Native radios group by `name`, and Octane's `defaultValue` becomes
// `checked=""` on the matching item.

#[component]
pub async fn choice_card() -> Result<impl View> {
    let plans = [
        ("plus", "plus-plan", "Plus", "For individuals and small teams."),
        ("pro", "pro-plan", "Pro", "For growing businesses."),
        ("enterprise", "enterprise-plan", "Enterprise", "For large teams and enterprises."),
    ];
    Ok(view! {
        radio_group(attrs: attributes! { class="max-w-sm" },
            #[key(value)]
            for (value, id, title, hint) in plans {
                field_label(attrs: attributes! { for=(id) },
                    field(orientation: FieldOrientation::Horizontal,
                        field_content(
                            field_title((title))
                            field_description((hint))
                        )
                        radio_group_item(attrs: attributes! {
                            value=(value) id=(id) name="choice-card-plan" checked=(value == "plus")
                        })
                    )
                )
            }
        )
    })
}

#[component]
pub async fn description() -> Result<impl View> {
    let options = [
        ("default", "desc-r1", "Default", "Standard spacing for most use cases."),
        ("comfortable", "desc-r2", "Comfortable", "More space between elements."),
        ("compact", "desc-r3", "Compact", "Minimal spacing for dense layouts."),
    ];
    Ok(view! {
        radio_group(attrs: attributes! { class="w-fit" },
            #[key(value)]
            for (value, id, text, hint) in options {
                field(orientation: FieldOrientation::Horizontal,
                    radio_group_item(attrs: attributes! {
                        value=(value) id=(id) name="description-density"
                        checked=(value == "comfortable")
                    })
                    field_content(
                        field_label(attrs: attributes! { for=(id) }, (text))
                        field_description((hint))
                    )
                )
            }
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        radio_group(attrs: attributes! { class="w-fit" },
            field(
                orientation: FieldOrientation::Horizontal,
                attrs: attributes! { data-disabled="true" },
                radio_group_item(attrs: attributes! {
                    value="option1" id="disabled-1" name="disabled-option" disabled=""
                })
                field_label(attrs: attributes! { for="disabled-1" class="font-base!" }, "Disabled")
            )
            field(orientation: FieldOrientation::Horizontal,
                radio_group_item(attrs: attributes! {
                    value="option2" id="disabled-2" name="disabled-option" checked=""
                })
                field_label(attrs: attributes! { for="disabled-2" class="font-base!" }, "Option 2")
            )
            field(orientation: FieldOrientation::Horizontal,
                radio_group_item(attrs: attributes! {
                    value="option3" id="disabled-3" name="disabled-option"
                })
                field_label(attrs: attributes! { for="disabled-3" class="font-base!" }, "Option 3")
            )
        )
    })
}

#[component]
pub async fn fieldset() -> Result<impl View> {
    let plans = [
        ("monthly", "plan-monthly", "Monthly ($9.99/month)"),
        ("yearly", "plan-yearly", "Yearly ($99.99/year)"),
        ("lifetime", "plan-lifetime", "Lifetime ($299.99)"),
    ];
    Ok(view! {
        field_set(attrs: attributes! { class="w-full max-w-xs" },
            field_legend(variant: FieldLegendVariant::Label, "Subscription Plan")
            field_description("Yearly and lifetime plans offer significant savings.")
            radio_group(
                #[key(value)]
                for (value, id, text) in plans {
                    field(orientation: FieldOrientation::Horizontal,
                        radio_group_item(attrs: attributes! {
                            value=(value) id=(id) name="subscription-plan"
                            checked=(value == "monthly")
                        })
                        field_label(attrs: attributes! { for=(id) class="font-base!" }, (text))
                    )
                }
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    let options = [
        ("default", "r1", "Default"),
        ("comfortable", "r2", "Comfortable"),
        ("compact", "r3", "Compact"),
    ];
    Ok(view! {
        radio_group(
            #[key(value)]
            for (value, id, text) in options {
                <div class="flex items-center space-x-2">
                    radio_group_item(attrs: attributes! {
                        value=(value) id=(id) name="density" checked=(value == "comfortable")
                    })
                    label(attrs: attributes! { for=(id) }, (text))
                </div>
            }
        )
    })
}

#[component]
pub async fn invalid() -> Result<impl View> {
    let options = [
        ("email", "invalid-email", "Email only"),
        ("sms", "invalid-sms", "SMS only"),
        ("both", "invalid-both", "Both Email & SMS"),
    ];
    Ok(view! {
        field_set(attrs: attributes! { class="w-full max-w-xs" },
            field_legend(variant: FieldLegendVariant::Label, "Notification Preferences")
            field_description("Choose how you want to receive notifications.")
            radio_group(
                #[key(value)]
                for (value, id, text) in options {
                    field(
                        orientation: FieldOrientation::Horizontal,
                        attrs: attributes! { data-invalid="true" },
                        radio_group_item(attrs: attributes! {
                            value=(value) id=(id) name="notification-preference"
                            aria-invalid="true" checked=(value == "email")
                        })
                        field_label(attrs: attributes! { for=(id) class="font-base!" }, (text))
                    )
                }
            )
        )
    })
}
