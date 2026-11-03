//! The Octane examples pass an `items` list to the root; here the steps are
//! simply the rendered items. Octane's `onSubmit` handlers read `FormData`;
//! the default example does the same in an inline `onsubmit`, since the
//! runtime cannot read form data, and the others keep their answers in
//! signals.

use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, expr, signal},
    view::{View, attributes, component, view},
};

use crate::components::progress::{progress, progress_label};
use crate::components::questionnaire::{
    QuestionnaireShortcuts, questionnaire, questionnaire_actions, questionnaire_choice,
    questionnaire_choice_description, questionnaire_choices, questionnaire_description,
    questionnaire_error, questionnaire_input, questionnaire_item, questionnaire_next,
    questionnaire_previous, questionnaire_progress, questionnaire_skip, questionnaire_submit,
    questionnaire_title,
};

/// Shows the submitted answers as JSON, like Octane's `handleSubmit`.
const SHOW_ANSWERS: &str = "event.preventDefault();\
    const data = new FormData(this);\
    const out = document.getElementById('questionnaire-demo-answers');\
    out.textContent = JSON.stringify({\
        framework: String(data.get('framework') ?? ''),\
        features: data.getAll('features').map(String),\
        feedback: String(data.get('feedback') ?? ''),\
    }, null, 2);\
    out.hidden = false;";

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            <div class="w-full">
                questionnaire(
                    shortcuts: QuestionnaireShortcuts::Letters,
                    attrs: attributes! { onsubmit=(SHOW_ANSWERS) },
                    questionnaire_progress()
                    questionnaire_item(name: "framework", required: true,
                        questionnaire_title("Which framework do you use?")
                        questionnaire_description("Pick the one you use most of the time.")
                        questionnaire_choices(
                            questionnaire_choice(value: "next",
                                "Next.js"
                                questionnaire_choice_description("App router with server components")
                            )
                            questionnaire_choice(value: "vite",
                                "Vite"
                                questionnaire_choice_description("Single page application")
                            )
                            questionnaire_choice(value: "astro",
                                "Astro"
                                questionnaire_choice_description("Content focused sites")
                            )
                        )
                        questionnaire_error()
                    )
                    questionnaire_item(name: "features", multiple: true,
                        questionnaire_title("What do you use from the library?")
                        questionnaire_description("Select all that apply, or skip this question.")
                        questionnaire_choices(
                            questionnaire_choice(value: "docs", "Documentation")
                            questionnaire_choice(value: "registry", "Registry")
                            questionnaire_choice(value: "themes", "Themes")
                        )
                        questionnaire_error()
                    )
                    questionnaire_item(name: "feedback",
                        questionnaire_title("Anything else?")
                        questionnaire_description("Tell us what you would like to see next.")
                        questionnaire_input(attrs: attributes! {
                            aria-label="Feedback"
                            placeholder="Type your answer..."
                        })
                        questionnaire_error()
                    )
                    questionnaire_actions(
                        questionnaire_previous()
                        questionnaire_skip()
                        questionnaire_next()
                        questionnaire_submit()
                    )
                )
            </div>
            <pre
                id="questionnaire-demo-answers"
                hidden=""
                class="w-full overflow-x-auto rounded-base border-2 border-border bg-secondary-background p-4 text-xs font-base"
            ></pre>
        </div>
    })
}

#[component]
pub async fn validation(cx: &Cx) -> Result<impl View> {
    let name = signal(cx, String::new);
    let email = signal(cx, String::new);
    let submitted = signal(cx, || false);

    // The runtime has no regular expressions, so this approximates Octane's
    // `/^[^\s@]+@[^\s@]+\.[^\s@]+$/` with string methods. Expressions have no
    // `&&` or `||`, so the checks are a chain of `if`s.
    let email_invalid = expr!({
        let value = email.get();
        if value.is_empty() {
            false
        } else if value.contains(" ") {
            true
        } else if value.starts_with("@") {
            true
        } else if value.ends_with(".") {
            true
        } else if value.contains("@") {
            !value.contains(".")
        } else {
            true
        }
    });

    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            questionnaire(
                attrs: attributes! {
                    @submit=$(|e: Event| {
                        e.prevent_default();
                        submitted.set(true);
                    })
                },
                questionnaire_progress()
                questionnaire_item(name: "name", required: true,
                    questionnaire_title("What should we call you?")
                    questionnaire_input(attrs: attributes! {
                        aria-label="Name"
                        placeholder="Your name"
                        autocomplete="name"
                        @input=$(|e: Event| name.set(e.target.value))
                    })
                    questionnaire_error("Enter your name to continue.")
                )
                questionnaire_item(name: "email", required: true, invalid: email_invalid,
                    questionnaire_title("Where can we reach you?")
                    questionnaire_description("We only use this to send you the results.")
                    questionnaire_input(attrs: attributes! {
                        type="email"
                        aria-label="Email"
                        placeholder="you@example.com"
                        autocomplete="email"
                        @input=$(|e: Event| email.set(e.target.value))
                    })
                    questionnaire_error(
                        $(if email.get().is_empty() {
                            "Enter your email to continue."
                        } else {
                            "Enter a valid email address."
                        })
                    )
                )
                questionnaire_actions(
                    questionnaire_previous()
                    questionnaire_next()
                    questionnaire_submit()
                )
            )
            <p class="text-sm font-base text-foreground" :hidden=$(!submitted.get())>
                $(("Thanks ", name.get(), ", we'll be in touch at ", email.get(), "."))
            </p>
        </div>
    })
}

#[component]
pub async fn controlled(cx: &Cx) -> Result<impl View> {
    // The current item's name, reported by the questionnaire's `itemchange`
    // event.
    let item = signal(cx, || "role".to_owned());
    let submitted = signal(cx, || false);

    let step = expr!(if item.get() == "role" {
        1
    } else if item.get() == "team" {
        2
    } else {
        3
    });
    let percent = expr!(if item.get() == "role" {
        100.0 / 3.0
    } else if item.get() == "team" {
        200.0 / 3.0
    } else {
        100.0
    });

    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-6">
            progress(value: percent, progress_label($(("Step ", step, " of 3"))))
            questionnaire(
                attrs: attributes! {
                    @itemchange=$(|e: Event| item.set(e.target.name))
                    @submit=$(|e: Event| {
                        e.prevent_default();
                        submitted.set(true);
                    })
                },
                questionnaire_item(name: "role", required: true,
                    questionnaire_title("What best describes you?")
                    questionnaire_choices(
                        questionnaire_choice(value: "designer", "Designer")
                        questionnaire_choice(value: "developer", "Developer")
                        questionnaire_choice(value: "founder", "Founder")
                    )
                    questionnaire_error()
                )
                questionnaire_item(name: "team", required: true,
                    questionnaire_title("How big is your team?")
                    questionnaire_choices(
                        questionnaire_choice(value: "solo", "Just me")
                        questionnaire_choice(value: "small", "2 to 10 people")
                        questionnaire_choice(value: "large", "More than 10 people")
                    )
                    questionnaire_error()
                )
                questionnaire_item(name: "usage", required: true,
                    questionnaire_title("Where will you use it?")
                    questionnaire_choices(
                        questionnaire_choice(value: "side", "Side projects")
                        questionnaire_choice(value: "work", "At work")
                        questionnaire_choice(value: "both", "Both")
                    )
                    questionnaire_error()
                )
                questionnaire_actions(
                    questionnaire_previous()
                    questionnaire_next()
                    questionnaire_submit()
                )
            )
            <p class="text-sm font-base text-foreground" :hidden=$(!submitted.get())>
                "All done, thanks for your answers!"
            </p>
        </div>
    })
}

#[component]
pub async fn single_step(cx: &Cx) -> Result<impl View> {
    // The picked choice, from the radios' bubbling `change` events, and the
    // vote, copied from it on submit.
    let choice = signal(cx, || "tabs".to_owned());
    let vote = signal(cx, String::new);

    Ok(view! {
        <div class="flex w-full max-w-md flex-col gap-4">
            questionnaire(
                shortcuts: QuestionnaireShortcuts::Numbers,
                attrs: attributes! {
                    @change=$(|e: Event| choice.set(e.target.value))
                    @submit=$(|e: Event| {
                        e.prevent_default();
                        vote.set(choice.get());
                    })
                },
                questionnaire_item(name: "indentation", required: true,
                    questionnaire_title("Tabs or spaces?")
                    questionnaire_description(
                        "Use the number keys to pick an answer, then press Enter."
                    )
                    questionnaire_choices(
                        questionnaire_choice(value: "tabs", checked: true, "Tabs")
                        questionnaire_choice(value: "spaces", "Spaces")
                        questionnaire_choice(value: "both", disabled: true,
                            "Both"
                            questionnaire_choice_description("Not available in this poll.")
                        )
                    )
                    questionnaire_error()
                )
                questionnaire_actions(questionnaire_submit("Vote"))
            )
            <p class="text-sm font-base text-foreground" :hidden=$(vote.get().is_empty())>
                $(("You voted for ", vote.get(), "."))
            </p>
        </div>
    })
}
