use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::button::button;
use crate::components::tabs::{TabsOrientation, tabs, tabs_content, tabs_list, tabs_trigger};

// The demo's Card, Label and Input belong to other components, so it uses
// plain elements with the same classes.

/// The classes of Octane's `Card`.
const CARD: &str = "group/card rounded-base flex flex-col shadow-shadow border-2 \
                    gap-(--card-spacing) py-(--card-spacing) border-border bg-background \
                    text-foreground font-base [--card-spacing:--spacing(6)]";
/// The classes of Octane's `CardHeader`.
const CARD_HEADER: &str = "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 \
                           px-(--card-spacing)";
/// The classes of Octane's `Label`.
const LABEL: &str = "text-sm font-heading leading-none";
/// The classes of Octane's `Input`.
const INPUT: &str = "flex h-10 w-full rounded-base border-2 border-border bg-secondary-background \
                     selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base \
                     text-foreground placeholder:text-foreground/50 focus-visible:outline-hidden \
                     focus-visible:ring-2 focus-visible:ring-black focus-visible:ring-offset-2";

/// Two panels that switch in the browser, without a server round trip.
#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        tabs(attrs: attributes! { class="max-w-[400px]" },
            // `grid!` beats the list's own `inline-flex` (class! doesn't merge).
            tabs_list(attrs: attributes! { class="grid! w-full grid-cols-2" },
                tabs_trigger(value: "account", active: true, "Account")
                tabs_trigger(value: "password", "Password")
            )
            tabs_content(value: "account", active: true,
                <div data-slot="card" class=(CARD)>
                    <div data-slot="card-header" class=(CARD_HEADER)>
                        <div class="font-heading leading-none">"Account"</div>
                        <div class="text-sm font-base">
                            "Make changes to your account here. Click save when you're done."
                        </div>
                    </div>
                    <div class="px-(--card-spacing) grid gap-6">
                        <div class="grid gap-3">
                            <label for="tabs-demo-name" class=(LABEL)>"Name"</label>
                            <input id="tabs-demo-name" value="Pedro Duarte" class=(INPUT)>
                        </div>
                        <div class="grid gap-3">
                            <label for="tabs-demo-username" class=(LABEL)>"Username"</label>
                            <input id="tabs-demo-username" value="@octanejs" class=(INPUT)>
                        </div>
                    </div>
                    <div class="flex items-center px-(--card-spacing)">
                        button(attrs: attributes! { class="w-full" }, "Save changes")
                    </div>
                </div>
            )
            tabs_content(value: "password",
                <div data-slot="card" class=(CARD)>
                    <div data-slot="card-header" class=(CARD_HEADER)>
                        <div class="font-heading leading-none">"Password"</div>
                        <div class="text-sm font-base">
                            "Change your password here. After saving, you'll be logged out."
                        </div>
                    </div>
                    <div class="px-(--card-spacing) grid gap-6">
                        <div class="grid gap-3">
                            <label for="tabs-demo-current" class=(LABEL)>"Current password"</label>
                            <input id="tabs-demo-current" type="password" class=(INPUT)>
                        </div>
                        <div class="grid gap-3">
                            <label for="tabs-demo-new" class=(LABEL)>"New password"</label>
                            <input id="tabs-demo-new" type="password" class=(INPUT)>
                        </div>
                    </div>
                    <div class="flex items-center px-(--card-spacing)">
                        button(attrs: attributes! { class="w-full" }, "Save password")
                    </div>
                </div>
            )
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        // `w-fit!` overrides the tabs' own `w-full`.
        tabs(attrs: attributes! { class="w-fit!" },
            tabs_list(
                tabs_trigger(value: "home", active: true, "Home")
                tabs_trigger(value: "settings", attrs: attributes! { disabled="" }, "Disabled")
            )
        )
    })
}

#[component]
pub async fn icons() -> Result<impl View> {
    Ok(view! {
        // `w-fit!` overrides the tabs' own `w-full`.
        tabs(attrs: attributes! { class="w-fit!" },
            tabs_list(
                tabs_trigger(value: "preview", active: true,
                    icon(data: iconify_icon!("lucide:app-window"))
                    "Preview"
                )
                tabs_trigger(value: "code",
                    icon(data: iconify_icon!("lucide:code"))
                    "Code"
                )
            )
        )
    })
}

#[component]
pub async fn vertical() -> Result<impl View> {
    Ok(view! {
        // `w-fit!` overrides the tabs' own `w-full`.
        tabs(orientation: TabsOrientation::Vertical, attrs: attributes! { class="w-fit!" },
            tabs_list(
                tabs_trigger(value: "account", active: true, "Account")
                tabs_trigger(value: "password", "Password")
                tabs_trigger(value: "notifications", "Notifications")
            )
        )
    })
}
