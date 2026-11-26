//! The profile page: the signed-in user's details and notification settings.

use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    avatar::{AvatarSize, avatar, avatar_fallback},
    button::button,
    input::input,
    label::label,
    switch::switch,
};

use super::{
    data::{NOTIFICATIONS, USER},
    shared::chart_card,
};

/// The form's fields: name, label, input type, autocomplete and value.
const FIELDS: [(&str, &str, &str, &str, &str); 4] = [
    ("name", "Name", "text", "name", USER.name),
    ("email", "Email", "email", "email", USER.email),
    ("role", "Role", "text", "off", USER.role),
    ("store", "Store", "text", "off", USER.store),
];

#[component]
pub async fn profile_page(cx: &Cx) -> Result<impl View> {
    // Set when the form is saved, cleared by the next edit.
    let saved = signal(cx, || false);

    Ok(view! {
        <div class="grid items-start gap-5 xl:grid-cols-3">
            chart_card(
                title: "Your details",
                description: "Shown to your team and on exported reports".to_owned(),
                class: "xl:col-span-2",
                <form
                    class="flex flex-col gap-5"
                    @input=$(|_e: Event| saved.set(false))
                    @submit=$(|e: Event| {
                        // Send the form to your API here.
                        e.prevent_default();
                        saved.set(true);
                    })
                >
                    <div class="flex items-center gap-4">
                        avatar(size: AvatarSize::Lg,
                            // `!` replaces the fallback's own background.
                            avatar_fallback(attrs: attributes! { class="bg-chart-3!" }, (USER.initials))
                        )
                        <div class="min-w-0">
                            <p class="truncate font-heading text-lg">(USER.name)</p>
                            <p class="truncate text-sm">(format!("Member since {}", USER.since))</p>
                        </div>
                    </div>
                    <div class="grid gap-5 sm:grid-cols-2">
                        for (name, text, kind, autocomplete, value) in FIELDS {
                            <div class="flex flex-col gap-2">
                                label(attrs: attributes! { for=(format!("profile-{name}")) }, (text))
                                input(attrs: attributes! {
                                    id=(format!("profile-{name}"))
                                    name=(name)
                                    type=(kind)
                                    autocomplete=(autocomplete)
                                    value=(value)
                                    required=""
                                })
                            </div>
                        }
                    </div>
                    <div class="flex items-center gap-4">
                        button(attrs: attributes! { type="submit" }, "Save changes")
                        <p aria-live="polite" class="text-sm">
                            $(if saved.get() { "Changes saved." } else { "" })
                        </p>
                    </div>
                </form>
            )

            chart_card(title: "Notifications", description: "Sent to your email".to_owned(),
                <ul class="flex flex-col gap-5">
                    for item in NOTIFICATIONS {
                        <li class="flex items-center justify-between gap-4">
                            <div class="flex flex-col gap-1.5">
                                label(attrs: attributes! { for=(format!("notify-{}", item.id)) }, (item.label))
                                <p id=(format!("notify-{}-text", item.id)) class="text-sm">(item.description)</p>
                            </div>
                            switch(attrs: attributes! {
                                id=(format!("notify-{}", item.id))
                                aria-describedby=(format!("notify-{}-text", item.id))
                                checked=(item.enabled)
                            })
                        </li>
                    }
                </ul>
            )
        </div>
    })
}
