use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::accordion::{
    accordion, accordion_content, accordion_item, accordion_trigger,
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        accordion(attrs: attributes! { class="w-full max-w-xl" },
            accordion_item(
                accordion_trigger("Is it accessible?")
                accordion_content("Yes. It adheres to the WAI-ARIA design pattern.")
            )
        )
    })
}

/// `(value, trigger, content)` for the basic example.
const BASIC_ITEMS: [(&str, &str, &str); 3] = [
    (
        "item-1",
        "How do I reset my password?",
        "Click on 'Forgot Password' on the login page, enter your email address, and we'll \
         send you a link to reset your password. The link will expire in 24 hours.",
    ),
    (
        "item-2",
        "Can I change my subscription plan?",
        "Yes, you can upgrade or downgrade your plan at any time from your account settings. \
         Changes will be reflected in your next billing cycle.",
    ),
    (
        "item-3",
        "What payment methods do you accept?",
        "We accept all major credit cards, PayPal, and bank transfers. All payments are \
         processed securely through our payment partners.",
    ),
];

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        accordion(attrs: attributes! { class="w-full max-w-xl space-y-4" },
            for (value, trigger, content) in BASIC_ITEMS {
                // One shared name: opening an item closes the others.
                accordion_item(
                    attrs: attributes! {
                        name="accordion-basic"
                        open=(value == "item-1")
                    },
                    accordion_trigger((trigger))
                    accordion_content((content))
                )
            }
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        accordion(attrs: attributes! { class="w-full max-w-xl space-y-4" },
            accordion_item(attrs: attributes! { name="accordion-disabled" },
                accordion_trigger("Can I access my account history?")
                accordion_content(
                    "Yes, you can view your complete account history including all \
                     transactions, plan changes, and support tickets in the Account History \
                     section of your dashboard."
                )
            )
            accordion_item(disabled: true, attrs: attributes! { name="accordion-disabled" },
                accordion_trigger("Premium feature information")
                accordion_content(
                    "This section contains information about premium features. Upgrade your \
                     plan to access this content."
                )
            )
            accordion_item(attrs: attributes! { name="accordion-disabled" },
                accordion_trigger("How do I update my email address?")
                accordion_content(
                    "You can update your email address in your account settings. You'll \
                     receive a verification email at your new address to confirm the change."
                )
            )
        )
    })
}

/// `(value, trigger, content)` for the multiple example.
const MULTIPLE_ITEMS: [(&str, &str, &str); 3] = [
    (
        "notifications",
        "Notification Settings",
        "Manage how you receive notifications. You can enable email alerts for updates or \
         push notifications for mobile devices.",
    ),
    (
        "privacy",
        "Privacy & Security",
        "Control your privacy settings and security preferences. Enable two-factor \
         authentication, manage connected devices, review active sessions, and configure data \
         sharing preferences. You can also download your data or delete your account.",
    ),
    (
        "billing",
        "Billing & Subscription",
        "View your current plan, payment history, and upcoming invoices. Update your payment \
         method, change your subscription tier, or cancel your subscription.",
    ),
];

#[component]
pub async fn multiple() -> Result<impl View> {
    Ok(view! {
        accordion(attrs: attributes! { class="w-full max-w-xl space-y-4" },
            for (value, trigger, content) in MULTIPLE_ITEMS {
                // No name: each item opens on its own.
                accordion_item(
                    attrs: attributes! { open=(value == "notifications") },
                    accordion_trigger((trigger))
                    accordion_content((content))
                )
            }
        )
    })
}
