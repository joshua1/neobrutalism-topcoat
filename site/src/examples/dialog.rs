//! The Octane demo uses `Input` and `Label`, which are ported in another
//! batch; they are plain elements with the same classes here.

use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::dialog::{
    dialog, dialog_close, dialog_content, dialog_description, dialog_footer, dialog_header,
    dialog_title, dialog_trigger,
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

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod \
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
    exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor \
    in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est \
    laborum.";

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <form>
            dialog_trigger(
                target: "dialog-demo",
                attrs: attributes! {
                    class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
                },
                "Edit Profile"
            )
            dialog(
                id: "dialog-demo",
                attrs: attributes! { aria-labelledby="dialog-demo-title" },
                // `!` overrides the panel's `sm:max-w-lg`.
                dialog_content(attrs: attributes! { class="sm:max-w-[425px]!" },
                    dialog_header(
                        dialog_title(attrs: attributes! { id="dialog-demo-title" }, "Edit profile")
                        dialog_description(
                            "Make changes to your profile here. Click save when you're done."
                        )
                    )
                    <div class="grid gap-4">
                        <div class="grid gap-3">
                            <label for="name-1" class=(LABEL)>"Name"</label>
                            <input id="name-1" name="name" value="Pedro Duarte" class=(INPUT)>
                        </div>
                        <div class="grid gap-3">
                            <label for="username-1" class=(LABEL)>"Username"</label>
                            <input id="username-1" name="username" value="@octanejs" class=(INPUT)>
                        </div>
                    </div>
                    dialog_footer(
                        dialog_close(
                            attrs: attributes! {
                                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
                            },
                            "Cancel"
                        )
                        button(attrs: attributes! { type="submit" }, "Save changes")
                    )
                )
            )
        </form>
    })
}

#[component]
pub async fn scrollable_content() -> Result<impl View> {
    Ok(view! {
        dialog_trigger(
            target: "dialog-scrollable",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
            },
            "Scrollable Content"
        )
        dialog(
            id: "dialog-scrollable",
            attrs: attributes! { aria-labelledby="dialog-scrollable-title" },
            // `!` overrides the panel's `sm:max-w-lg`.
            dialog_content(attrs: attributes! { class="sm:max-w-[425px]!" },
                dialog_header(
                    dialog_title(
                        attrs: attributes! { id="dialog-scrollable-title" },
                        "Scrollable Content"
                    )
                    dialog_description("This is a dialog with scrollable content.")
                )
                <div class="-mx-6 max-h-[500px] overflow-y-auto px-6 text-sm">
                    <h4 class="mb-4 text-lg leading-none font-heading">"Lorem Ipsum"</h4>
                    for _ in 0..10 {
                        <p class="mb-4 leading-normal">(LOREM)</p>
                    }
                </div>
            )
        )
    })
}

#[component]
pub async fn sticky_footer() -> Result<impl View> {
    Ok(view! {
        dialog_trigger(
            target: "dialog-sticky-footer",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
            },
            "Sticky Footer"
        )
        dialog(
            id: "dialog-sticky-footer",
            attrs: attributes! { aria-labelledby="dialog-sticky-footer-title" },
            dialog_content(attrs: attributes! { class="sm:max-w-lg" },
                dialog_header(
                    dialog_title(
                        attrs: attributes! { id="dialog-sticky-footer-title" },
                        "Scrollable Content"
                    )
                    dialog_description("This is a dialog with scrollable content.")
                )
                <div class="-mx-6 max-h-[500px] overflow-y-auto px-6 text-sm">
                    <h4 class="mb-4 text-lg leading-none font-medium">"Lorem Ipsum"</h4>
                    for _ in 0..10 {
                        <p class="mb-4 leading-normal">(LOREM)</p>
                    }
                </div>
                dialog_footer(
                    dialog_close(
                        attrs: attributes! {
                            class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
                        },
                        "Close"
                    )
                )
            )
        )
    })
}
