//! Like the Octane examples, each one renders its own `toaster`, which
//! defines the page's `toast()` function; the buttons call it from `onclick`.
//! In an app, render one `toaster` in the root layout.

use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::button::{ButtonVariant, button};
use crate::components::toast::toaster;

#[component]
pub async fn demo() -> Result<impl View> {
    // The action closes the toast after running its (optional) `onClick`.
    Ok(view! {
        toaster()
        button(
            attrs: attributes! {
                onclick="toast({
                    title: 'Event created',
                    description: 'Sunday, December 3 at 9:00 AM',
                    action: { label: 'Undo' },
                })"
            },
            "Show Toast"
        )
    })
}

#[component]
pub async fn types() -> Result<impl View> {
    Ok(view! {
        toaster()
        <div class="flex flex-wrap justify-center gap-2">
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! { onclick="toast({ description: 'Event has been created.' })" },
                "Default"
            )
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! {
                    onclick="toast({ variant: 'success', description: 'Event has been created.' })"
                },
                "Success"
            )
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! {
                    onclick="toast({ variant: 'info', description: 'Arrive 10 minutes before the event.' })"
                },
                "Info"
            )
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! {
                    onclick="toast({
                        variant: 'warning',
                        description: 'The event cannot start before 8:00 AM.',
                    })"
                },
                "Warning"
            )
            button(
                variant: ButtonVariant::Neutral,
                attrs: attributes! {
                    onclick="toast({
                        variant: 'error',
                        description: 'The event could not be created.',
                        priority: 'high',
                    })"
                },
                "Error"
            )
        </div>
    })
}

#[component]
pub async fn action() -> Result<impl View> {
    Ok(view! {
        toaster()
        button(
            variant: ButtonVariant::Neutral,
            attrs: attributes! {
                onclick="toast({ title: 'Event created', action: { label: 'Undo' } })"
            },
            "Show Toast"
        )
    })
}

/// Resolves after two seconds, standing in for a request.
const CREATE_EVENT: &str = r#"toast.promise(
    new Promise((resolve) => setTimeout(() => resolve({ name: 'Event' }), 2000)),
    {
        loading: 'Creating event…',
        success: (data) => `${data.name} created.`,
        error: 'Could not create event.',
    },
)"#;

#[component]
pub async fn promise() -> Result<impl View> {
    Ok(view! {
        toaster()
        button(
            variant: ButtonVariant::Neutral,
            attrs: attributes! { onclick=(CREATE_EVENT) },
            "Create Event"
        )
    })
}
