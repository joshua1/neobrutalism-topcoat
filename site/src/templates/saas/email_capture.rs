//! An email field and button that turn into a thank-you note when sent.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, class, component, element_id, view},
};

use crate::components::{
    button::{ButtonSize, ButtonVariant, button},
    input::input,
};

/// The form checks the address natively (`type="email"`, `required`); a
/// valid one swaps the form for the note, in the browser. Nothing is sent.
/// `class` goes on both the form and the note.
#[component]
pub async fn email_capture(
    cx: &Cx,
    #[default("Get started")] cta: &str,
    #[default] variant: ButtonVariant,
    #[default] class: &str,
) -> Result<impl View> {
    let id = element_id(cx);
    let submitted = signal(cx, || false);

    Ok(view! {
        <form
            :hidden=$(submitted.get())
            @submit=$(|e: Event| {
                e.prevent_default();
                submitted.set(true);
            })
            class=(class!("flex w-full max-w-md flex-col gap-3 sm:flex-row", class))
        >
            <label for=(&id) class="sr-only">"Work email"</label>
            input(attrs: attributes! {
                id=(&id)
                type="email"
                required=""
                placeholder="Work email"
                class="h-11"
            })
            button(
                variant: variant,
                size: ButtonSize::Lg,
                attrs: attributes! { type="submit" class="shrink-0" },
                (cta)
            )
        </form>
        <p
            role="status"
            :hidden=$(!submitted.get())
            class=(class!(
                "flex h-11 w-fit items-center gap-2 rounded-base border-2 border-border bg-chart-4 px-4 text-sm font-heading text-foreground shadow-shadow",
                class,
            ))
        >
            icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="size-4" })
            " You're on the list. Check your inbox."
        </p>
    })
}
