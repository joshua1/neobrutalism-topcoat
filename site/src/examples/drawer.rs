//! Swiping, snap points and the scaled stack of nested drawers are not
//! ported (see the `drawer` component docs), so `snap_points` shows a plain
//! bottom drawer with its handle. Octane's `useIsMobile` choices are made in
//! CSS (`responsive`) or fixed to the desktop layout (`nested`). `Input` and
//! `Label` are ported in another batch; they are plain elements here.

use topcoat::{
    Result,
    view::{Attributes, Class, StaticClass, View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::dialog::{
    dialog, dialog_content, dialog_description, dialog_header, dialog_title, dialog_trigger,
};
use crate::components::drawer::{
    DrawerSwipeDirection, drawer, drawer_close, drawer_content, drawer_description,
    drawer_footer, drawer_header, drawer_title, drawer_trigger,
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

/// A placeholder block that fills a side drawer or sits in a bottom one.
const PLACEHOLDER: &str = "rounded-base bg-secondary-background \
    group-data-[swipe-axis=x]/drawer-popup:size-full group-data-[swipe-axis=y]/drawer-popup:h-80 \
    group-data-[swipe-axis=y]/drawer-popup:w-full";

/// Classes for a trigger or close button styled as a neutral button.
fn neutral() -> Class<(StaticClass, StaticClass, StaticClass)> {
    button_variants(ButtonVariant::Neutral, ButtonSize::Default)
}

/// Classes for a trigger or close button styled as a default button.
fn primary() -> Class<(StaticClass, StaticClass, StaticClass)> {
    button_variants(ButtonVariant::Default, ButtonSize::Default)
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-demo", attrs: attributes! { class=(primary()) }, "Open")
        drawer(
            id: "drawer-demo",
            attrs: attributes! { aria-labelledby="drawer-demo-title" },
            drawer_content(
                <div class="mx-auto w-[300px]">
                    drawer_header(
                        drawer_title(
                            attrs: attributes! { id="drawer-demo-title" },
                            "Are you absolutely sure?"
                        )
                        drawer_description("This action cannot be undone.")
                    )
                    // `grid!` overrides the footer's `flex`.
                    drawer_footer(attrs: attributes! { class="grid! grid-cols-2" },
                        button(variant: ButtonVariant::NoShadow, "Submit")
                        drawer_close(
                            attrs: attributes! {
                                // `!` overrides the button's main fill and text color.
                                class=(class!(
                                    button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                                    "bg-secondary-background! text-foreground!",
                                ))
                            },
                            "Cancel"
                        )
                    )
                </div>
            )
        )
    })
}

#[component]
pub async fn nested() -> Result<impl View> {
    // Octane uses a bottom drawer with a handle on mobile and a right drawer
    // without one on desktop; this keeps the desktop layout.
    let nested_placeholder = "bg-secondary-background \
        group-data-[swipe-axis=x]/drawer-popup:size-full \
        group-data-[swipe-axis=y]/drawer-popup:aspect-video \
        group-data-[swipe-axis=y]/drawer-popup:w-full";

    Ok(view! {
        drawer_trigger(target: "drawer-nested-1", attrs: attributes! { class=(neutral()) }, "Open Drawer")
        drawer(id: "drawer-nested-1",
            drawer_content(swipe_direction: DrawerSwipeDirection::Right,
                drawer_header(
                    drawer_title("Drawer")
                    drawer_description("Open another drawer from the same direction.")
                )
                <div class="flex-1 p-4"><div class=(nested_placeholder)></div></div>
                drawer_footer(
                    drawer_trigger(target: "drawer-nested-2", attrs: attributes! { class=(neutral()) }, "Open Nested Drawer")
                    drawer(id: "drawer-nested-2",
                        drawer_content(swipe_direction: DrawerSwipeDirection::Right,
                            drawer_header(
                                drawer_title("Nested Drawer")
                                drawer_description(
                                    "The parent drawer stays mounted behind this one."
                                )
                            )
                            <div class="flex-1 p-4"><div class=(nested_placeholder)></div></div>
                            drawer_footer(
                                drawer_trigger(
                                    target: "drawer-nested-3",
                                    attrs: attributes! { class=(neutral()) },
                                    "Open Third Drawer"
                                )
                                drawer(id: "drawer-nested-3",
                                    drawer_content(swipe_direction: DrawerSwipeDirection::Right,
                                        drawer_header(
                                            drawer_title("Third Drawer")
                                            drawer_description(
                                                "Two drawers are stacked behind this one."
                                            )
                                        )
                                        <div class="flex-1 p-4">
                                            <div class=(nested_placeholder)></div>
                                        </div>
                                        drawer_footer(
                                            drawer_trigger(
                                                target: "drawer-nested-4",
                                                attrs: attributes! { class=(neutral()) },
                                                "Open Fourth Drawer"
                                            )
                                            drawer(id: "drawer-nested-4",
                                                drawer_content(
                                                    swipe_direction: DrawerSwipeDirection::Right,
                                                    drawer_header(
                                                        drawer_title("Fourth Drawer")
                                                        drawer_description(
                                                            "This is the frontmost drawer in the \
                                                             stack."
                                                        )
                                                    )
                                                    <div class="flex-1 p-4">
                                                        <div class=(nested_placeholder)></div>
                                                    </div>
                                                    drawer_footer(
                                                        drawer_close(attrs: attributes! { class=(neutral()) }, "Close")
                                                    )
                                                )
                                            )
                                            drawer_close(attrs: attributes! { class=(neutral()) }, "Close")
                                        )
                                    )
                                )
                                drawer_close(attrs: attributes! { class=(neutral()) }, "Close")
                            )
                        )
                    )
                    drawer_close(attrs: attributes! { class=(neutral()) }, "Close")
                )
            )
        )
    })
}

#[component]
pub async fn non_modal() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-non-modal", modal: false, attrs: attributes! { class=(neutral()) }, "Non Modal")
        drawer(id: "drawer-non-modal", modal: false, disable_pointer_dismissal: true,
            drawer_content(swipe_direction: DrawerSwipeDirection::Right,
                drawer_header(drawer_title("Non Modal Drawer"))
                <div class="flex-1 p-4"><div class=(PLACEHOLDER)></div></div>
                drawer_footer(drawer_close(attrs: attributes! { class=(primary()) }, "Close"))
            )
        )
    })
}

#[component]
pub async fn position() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-position", attrs: attributes! { class=(neutral()) }, "Open Left Drawer")
        drawer(id: "drawer-position",
            drawer_content(swipe_direction: DrawerSwipeDirection::Left,
                drawer_header(
                    drawer_title("Move Goal")
                    drawer_description("Set your daily activity goal.")
                )
                <div class="flex-1 p-4">
                    <div class="size-full rounded-base bg-secondary-background"></div>
                </div>
                drawer_footer(drawer_close(attrs: attributes! { class=(primary()) }, "Close"))
            )
        )
    })
}

/// The profile form shared by both halves of [`responsive`]. `prefix` keeps
/// the field ids unique, since both copies are on the page.
#[component]
async fn profile_form(prefix: &str, #[default] mut attrs: Attributes) -> Result<impl View> {
    let email = format!("{prefix}-email");
    let username = format!("{prefix}-username");

    Ok(view! {
        <form class=(class!("grid items-start gap-6", attrs.remove("class"))) (attrs)>
            <div class="grid gap-3">
                <label for=(&email) class=(LABEL)>"Email"</label>
                <input type="email" id=(&email) value="shadcn@example.com" class=(INPUT)>
            </div>
            <div class="grid gap-3">
                <label for=(&username) class=(LABEL)>"Username"</label>
                <input id=(&username) value="@shadcn" class=(INPUT)>
            </div>
            button(attrs: attributes! { type="submit" }, "Save changes")
        </form>
    })
}

#[component]
pub async fn responsive() -> Result<impl View> {
    // Octane renders one or the other with `useIsMobile`; here both are on the
    // page and CSS shows the dialog's trigger from `md` up and the drawer's
    // below it.
    Ok(view! {
        <span class="hidden md:contents">
            dialog_trigger(target: "drawer-responsive-dialog", attrs: attributes! { class=(neutral()) }, "Edit Profile")
        </span>
        <span class="md:hidden">
            drawer_trigger(target: "drawer-responsive-drawer", attrs: attributes! { class=(neutral()) }, "Edit Profile")
        </span>
        dialog(id: "drawer-responsive-dialog",
            // `!` overrides the panel's `sm:max-w-lg`.
            dialog_content(attrs: attributes! { class="sm:max-w-[425px]!" },
                dialog_header(
                    dialog_title("Edit profile")
                    dialog_description(
                        "Make changes to your profile here. Click save when you're done."
                    )
                )
                profile_form(prefix: "drawer-responsive-dialog")
            )
        )
        drawer(id: "drawer-responsive-drawer",
            drawer_content(
                // `!` overrides the header's `text-center sm:text-left`.
                drawer_header(attrs: attributes! { class="text-left!" },
                    drawer_title("Edit profile")
                    drawer_description(
                        "Make changes to your profile here. Click save when you're done."
                    )
                )
                profile_form(
                    prefix: "drawer-responsive-drawer",
                    attrs: attributes! { class="p-4" }
                )
            )
        )
    })
}

#[component]
pub async fn scrollable_content() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-scrollable", attrs: attributes! { class=(primary()) }, "Scrollable Content")
        drawer(id: "drawer-scrollable",
            drawer_content(swipe_direction: DrawerSwipeDirection::Right,
                drawer_header(
                    drawer_title("Move Goal")
                    drawer_description("Set your daily activity goal.")
                )
                <div class="overflow-y-auto px-4 text-sm">
                    <h4 class="mb-4 text-lg leading-none font-heading">"Lorem Ipsum"</h4>
                    for _ in 0..10 {
                        <p class="mb-4 leading-normal">(LOREM)</p>
                    }
                </div>
                drawer_footer(
                    button("Submit")
                    drawer_close(attrs: attributes! { class=(neutral()) }, "Cancel")
                )
            )
        )
    })
}

#[component]
pub async fn snap_points() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-snap-points", attrs: attributes! { class=(neutral()) }, "Open Snap Drawer")
        drawer(id: "drawer-snap-points",
            drawer_content(show_swipe_handle: Some(true),
                drawer_header(
                    drawer_title("Snap points")
                    drawer_description(
                        "Drag the drawer to snap between a compact peek and a near full-height \
                         view."
                    )
                )
                <div class="flex-1 p-4"><div class=(PLACEHOLDER)></div></div>
                drawer_footer(drawer_close(attrs: attributes! { class=(primary()) }, "Close"))
            )
        )
    })
}

#[component]
pub async fn swipe_handle() -> Result<impl View> {
    Ok(view! {
        drawer_trigger(target: "drawer-swipe-handle", attrs: attributes! { class=(neutral()) }, "Open Drawer")
        drawer(id: "drawer-swipe-handle",
            drawer_content(show_swipe_handle: Some(true),
                drawer_header(
                    drawer_title("Drawer")
                    drawer_description("Drawer with a swipe handle.")
                )
                <div class="flex-1 p-4"><div class=(PLACEHOLDER)></div></div>
                drawer_footer(drawer_close(attrs: attributes! { class=(primary()) }, "Close"))
            )
        )
    })
}
