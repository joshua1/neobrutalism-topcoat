use std::collections::HashMap;

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::query_params,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

// Six examples share their names with the component they show, which they
// then reach through this alias: `ui::input::input(..)`.
use crate::components as ui;
use crate::components::{
    button::{ButtonSize, ButtonVariant, button, button_variants},
    card::{card, card_content, card_description, card_footer, card_header, card_title},
    form::{FormField, form_description, form_item, form_label, form_message},
    input_group::{
        InputGroupAddonAlign, InputGroupButtonSize, input_group, input_group_addon,
        input_group_button, input_group_input, input_group_text, input_group_textarea,
    },
    radio_group::radio_group_item,
    select::{select_content, select_item, select_separator},
};

// These examples run on the docs page, which only answers GET, so each form
// submits to the page itself with `method="get"` and the example reads the
// values back from the query string with `#[query_params]`. An app posts the
// form and reads it with `Form<T>` instead (see `FormField`); the validation
// and rendering are the same. Every example shares the page's URL, so a
// hidden `form` field names the one that was submitted, and `action` keeps
// the page scrolled to the example's heading.
//
// The controls carry the native constraints that match each rule (`required`,
// `minlength`, `pattern`), but the forms are `novalidate` so that the
// server's messages show; the first example lets the browser check instead.
// "Reset" links to the page without the submitted values (an app links to
// its form page with `href!`).
// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

/// Field name to validation message. Each field reports its first failing
/// rule only.
type Errors = HashMap<&'static str, &'static str>;

/// The hidden `form` field, naming the example that was submitted.
#[query_params]
struct Submitted {
    form: Option<String>,
}

/// Whether the example whose `<form>` has this `id` was submitted.
fn submitted(cx: &Cx, id: &str) -> bool {
    query_params::<Submitted>(cx).is_ok_and(|query| query.form.as_deref() == Some(id))
}

/// The values of a valid submission, which the Octane examples show in a
/// toast.
#[component]
async fn submitted_values(values: String) -> Result<impl View> {
    Ok(view! {
        <div role="status" class="mt-6 grid gap-2 text-sm font-base">
            <p class="font-heading">"You submitted the following values:"</p>
            <code class="mt-2 block w-full overflow-x-auto rounded-base border-2 border-border bg-secondary-background p-3 text-xs whitespace-pre">
                (values)
            </code>
        </div>
    })
}

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct Account {
    username: String,
}

fn validate_account(account: &Account) -> Errors {
    let mut errors = Errors::new();
    if account.username.chars().count() < 2 {
        errors.insert("username", "Username must be at least 2 characters.");
    }
    errors
}

#[component]
pub async fn demo(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<Account>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-demo"));
    let errors = submission.map(validate_account).unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let username = FormField::new("form-demo-username", errors.get("username").copied());

    Ok(view! {
        // No `novalidate`: `required` and `minlength` match the rule, so the
        // browser checks it first and the server's check is the backstop.
        <form id="form-demo" method="get" class="space-y-8">
            <input type="hidden" name="form" value="form-demo">
            form_item(
                form_label(field: username, "Username")
                ui::input::input(attrs: attributes! {
                    name="username"
                    value=(submission.map_or("", |account| account.username.as_str()))
                    placeholder="ekmas"
                    required=""
                    minlength="2"
                    (username.control(cx))
                })
                form_description(field: username, "This is your public display name.")
                form_message(field: username)
            )
            button(attrs: attributes! { type="submit" }, "Submit")
            if let Some(saved) = saved {
                submitted_values(values: format!("{saved:#?}"))
            }
        </form>
    })
}

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct BugReport {
    title: String,
    description: String,
}

fn validate_bug_report(report: &BugReport) -> Errors {
    let mut errors = Errors::new();
    let title = report.title.chars().count();
    if title < 5 {
        errors.insert("title", "Bug title must be at least 5 characters.");
    } else if title > 32 {
        errors.insert("title", "Bug title must be at most 32 characters.");
    }
    let description = report.description.chars().count();
    if description < 20 {
        errors.insert("description", "Description must be at least 20 characters.");
    } else if description > 100 {
        errors.insert("description", "Description must be at most 100 characters.");
    }
    errors
}

#[component]
pub async fn basic(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<BugReport>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-basic"));
    let errors = submission.map(validate_bug_report).unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let values = submission.cloned().unwrap_or_default();
    let title = FormField::new("form-basic-title", errors.get("title").copied());
    let description = FormField::new("form-basic-description", errors.get("description").copied());
    // Counts as the user types, like Octane's `field.value.length`.
    let initial_length = values.description.len();
    let length = signal(cx, move || initial_length);

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Bug Report")
                card_description("Help us improve by reporting bugs you encounter.")
            )
            card_content(
                <form
                    id="form-basic"
                    method="get"
                    action="#basic"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-basic">
                    form_item(
                        form_label(field: title, "Bug Title")
                        ui::input::input(attrs: attributes! {
                            name="title"
                            value=(values.title.as_str())
                            placeholder="Login button not working on mobile"
                            autocomplete="off"
                            required=""
                            minlength="5"
                            (title.control(cx))
                        })
                        form_message(field: title)
                    )
                    form_item(
                        form_label(field: description, "Description")
                        input_group(
                            input_group_textarea(
                                attrs: attributes! {
                                    name="description"
                                    placeholder="I'm having an issue with the login button on mobile."
                                    rows="6"
                                    class="min-h-24 resize-none"
                                    required=""
                                    minlength="20"
                                    @input=$(|e: Event| length.set(e.target.value.len()))
                                    (description.control(cx))
                                },
                                (values.description.as_str())
                            )
                            input_group_addon(align: InputGroupAddonAlign::BlockEnd,
                                input_group_text(attrs: attributes! { class="tabular-nums" },
                                    $(length.get()) "/100 characters"
                                )
                            )
                        )
                        form_description(field: description,
                            "Include steps to reproduce, expected behavior, and what actually happened."
                        )
                        form_message(field: description)
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#basic" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-basic" }, "Submit")
            )
        )
    })
}

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct ProfileSettings {
    username: String,
}

fn validate_profile_settings(settings: &ProfileSettings) -> Errors {
    let mut errors = Errors::new();
    let username = settings.username.as_str();
    let length = username.chars().count();
    if length < 3 {
        errors.insert("username", "Username must be at least 3 characters.");
    } else if length > 10 {
        errors.insert("username", "Username must be at most 10 characters.");
    } else if !username
        .chars()
        .all(|char| char.is_ascii_alphanumeric() || char == '_')
    {
        errors.insert(
            "username",
            "Username can only contain letters, numbers, and underscores.",
        );
    }
    errors
}

#[component]
pub async fn input(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<ProfileSettings>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-input"));
    let errors = submission
        .map(validate_profile_settings)
        .unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let username = FormField::new("form-input-username", errors.get("username").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Profile Settings")
                card_description("Update your profile information below.")
            )
            card_content(
                <form
                    id="form-input"
                    method="get"
                    action="#input"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-input">
                    form_item(
                        form_label(field: username, "Username")
                        ui::input::input(attrs: attributes! {
                            name="username"
                            value=(submission.map_or("", |settings| settings.username.as_str()))
                            placeholder="shadcn"
                            autocomplete="username"
                            required=""
                            minlength="3"
                            pattern="[a-zA-Z0-9_]+"
                            (username.control(cx))
                        })
                        form_description(field: username,
                            "This is your public display name. Must be between 3 and 10 \
                             characters. Must only contain letters, numbers, and underscores."
                        )
                        form_message(field: username)
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#input" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-input" }, "Save")
            )
        )
    })
}

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct Personalization {
    about: String,
}

fn validate_personalization(personalization: &Personalization) -> Errors {
    let mut errors = Errors::new();
    let length = personalization.about.chars().count();
    if length < 10 {
        errors.insert("about", "Please provide at least 10 characters.");
    } else if length > 200 {
        errors.insert("about", "Please keep it under 200 characters.");
    }
    errors
}

#[component]
pub async fn textarea(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<Personalization>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-textarea"));
    let errors = submission.map(validate_personalization).unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let about = FormField::new("form-textarea-about", errors.get("about").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Personalization")
                card_description("Customize your experience by telling us more about yourself.")
            )
            card_content(
                <form
                    id="form-textarea"
                    method="get"
                    action="#textarea"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-textarea">
                    form_item(
                        form_label(field: about, "More about you")
                        ui::textarea::textarea(
                            attrs: attributes! {
                                name="about"
                                placeholder="I'm a software engineer..."
                                class="min-h-[120px]!"
                                required=""
                                minlength="10"
                                (about.control(cx))
                            },
                            (submission.map_or("", |personalization| personalization.about.as_str()))
                        )
                        form_description(field: about,
                            "Tell us more about yourself. This will be used to help us \
                             personalize your experience."
                        )
                        form_message(field: about)
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#textarea" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-textarea" }, "Save")
            )
        )
    })
}

/// The spoken languages, as `(value, label)`.
const SPOKEN_LANGUAGES: [(&str, &str); 7] = [
    ("en", "English"),
    ("es", "Spanish"),
    ("fr", "French"),
    ("de", "German"),
    ("it", "Italian"),
    ("zh", "Chinese"),
    ("ja", "Japanese"),
];

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct LanguagePreferences {
    language: String,
}

fn validate_language_preferences(preferences: &LanguagePreferences) -> Errors {
    let mut errors = Errors::new();
    let language = preferences.language.as_str();
    if language == "auto" {
        errors.insert(
            "language",
            "Auto-detection is not allowed. Please select a specific language.",
        );
    } else if !SPOKEN_LANGUAGES.iter().any(|(value, _)| *value == language) {
        errors.insert("language", "Please select your spoken language.");
    }
    errors
}

#[component]
pub async fn select(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<LanguagePreferences>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-select"));
    let errors = submission
        .map(validate_language_preferences)
        .unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let chosen = submission.map_or("", |preferences| preferences.language.as_str());
    let language = FormField::new("form-select-language", errors.get("language").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-lg" },
            card_header(
                card_title("Language Preferences")
                card_description("Select your preferred spoken language.")
            )
            card_content(
                <form
                    id="form-select"
                    method="get"
                    action="#select"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-select">
                    form_item(
                        attrs: attributes! {
                            class="flex! flex-col gap-4! sm:flex-row sm:items-center sm:justify-between"
                        },
                        <div class="grid gap-2">
                            form_label(field: language, "Spoken Language")
                            form_description(field: language,
                                "For best results, select the language you speak."
                            )
                            form_message(field: language)
                        </div>
                        ui::select::select(
                            placeholder: "Select",
                            attrs: attributes! {
                                name="language"
                                class="min-w-[120px] sm:w-auto"
                                required=""
                                (language.control(cx))
                            },
                            select_content(
                                select_item(attrs: attributes! { value="auto" selected=(chosen == "auto") },
                                    "Auto"
                                )
                                select_separator()
                                #[key(value)]
                                for (value, name) in SPOKEN_LANGUAGES {
                                    select_item(attrs: attributes! { value=(value) selected=(chosen == value) },
                                        (name)
                                    )
                                }
                            )
                        )
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#select" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-select" }, "Save")
            )
        )
    })
}

/// The notification types, as `(name, label)`.
const TASKS: [(&str, &str); 2] = [
    ("push", "Push notifications"),
    ("email", "Email notifications"),
];

/// A checked checkbox submits `name=true` (its `value`); an unchecked one
/// submits nothing, which `#[serde(default)]` reads as `false`.
#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct Notifications {
    push: bool,
    email: bool,
}

impl Notifications {
    /// Whether the task named `name` is on.
    fn task(&self, name: &str) -> bool {
        match name {
            "push" => self.push,
            "email" => self.email,
            _ => false,
        }
    }
}

fn validate_notifications(notifications: &Notifications) -> Errors {
    let mut errors = Errors::new();
    if !notifications.push && !notifications.email {
        errors.insert("tasks", "Please select at least one notification type.");
    }
    errors
}

#[component]
pub async fn checkbox(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<Notifications>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-checkbox"));
    let errors = submission.map(validate_notifications).unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let values = submission.cloned().unwrap_or_default();
    let responses = FormField::new("form-checkbox-responses", None);
    let responses_push = FormField::new("form-checkbox-responses-push", None);
    let tasks = FormField::new("form-checkbox-tasks", errors.get("tasks").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Notifications")
                card_description("Manage your notification preferences.")
            )
            card_content(
                <form
                    id="form-checkbox"
                    method="get"
                    action="#checkbox"
                    novalidate=""
                    class="grid gap-8"
                >
                    <input type="hidden" name="form" value="form-checkbox">
                    form_item(
                        form_label(field: responses, "Responses")
                        form_description(field: responses,
                            "Get notified for requests that take time, like research or image \
                             generation."
                        )
                        form_item(attrs: attributes! { class="flex! items-center gap-2" },
                            ui::checkbox::checkbox(attrs: attributes! {
                                checked=""
                                disabled=""
                                (responses_push.control(cx))
                            })
                            form_label(field: responses_push, attrs: attributes! { class="font-base!" },
                                "Push notifications"
                            )
                        )
                        form_message(field: responses)
                    )
                    form_item(
                        form_label(field: tasks, "Tasks")
                        form_description(field: tasks,
                            "Get notified when tasks you've created have updates."
                        )
                        #[key(name)]
                        for (name, title) in TASKS {
                            let id = format!("form-checkbox-{name}");
                            let task = FormField::new(&id, tasks.error);
                            form_item(attrs: attributes! { class="flex! items-center gap-2" },
                                ui::checkbox::checkbox(attrs: attributes! {
                                    name=(name)
                                    value="true"
                                    checked=(values.task(name))
                                    (task.control(cx))
                                })
                                form_label(field: task, attrs: attributes! { class="font-base!" },
                                    (title)
                                )
                            )
                        }
                        form_message(field: tasks)
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#checkbox" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-checkbox" }, "Save")
            )
        )
    })
}

/// The plans, as `(id, title, description)`.
const PLANS: [(&str, &str, &str); 3] = [
    (
        "starter",
        "Starter (100K tokens/month)",
        "For everyday use with basic features.",
    ),
    (
        "pro",
        "Pro (1M tokens/month)",
        "For advanced AI usage with more features.",
    ),
    (
        "enterprise",
        "Enterprise (Unlimited tokens)",
        "For large teams and heavy usage.",
    ),
];

#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct SubscriptionPlan {
    plan: String,
}

fn validate_subscription_plan(subscription: &SubscriptionPlan) -> Errors {
    let mut errors = Errors::new();
    if !PLANS.iter().any(|(id, ..)| *id == subscription.plan) {
        errors.insert("plan", "You must select a subscription plan to continue.");
    }
    errors
}

#[component]
pub async fn radio_group(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<SubscriptionPlan>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-radio-group"));
    let errors = submission
        .map(validate_subscription_plan)
        .unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let chosen = submission.map_or("", |subscription| subscription.plan.as_str());
    let plan = FormField::new("form-radio-group-plan", errors.get("plan").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Subscription Plan")
                card_description("See pricing and features for each plan.")
            )
            card_content(
                <form
                    id="form-radio-group"
                    method="get"
                    action="#radio_group"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-radio-group">
                    form_item(
                        form_label(field: plan, "Plan")
                        form_description(field: plan,
                            "You can upgrade or downgrade your plan at any time."
                        )
                        ui::radio_group::radio_group(
                            attrs: attributes! { class="gap-3" (plan.control(cx)) },
                            #[key(id)]
                            for (id, title, detail) in PLANS {
                                let item_id = format!("form-radio-group-{id}");
                                let option = FormField::new(&item_id, plan.error);
                                form_item(
                                    attrs: attributes! {
                                        class="flex! items-start justify-between gap-4! rounded-base \
                                            border-2 border-border bg-secondary-background p-4"
                                    },
                                    <div class="grid gap-1.5">
                                        form_label(field: option, (title))
                                        form_description(field: option, (detail))
                                    </div>
                                    radio_group_item(attrs: attributes! {
                                        name="plan"
                                        value=(id)
                                        required=""
                                        checked=(chosen == id)
                                        (option.control(cx))
                                    })
                                )
                            }
                        )
                        form_message(field: plan)
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#radio_group" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-radio-group" }, "Save")
            )
        )
    })
}

/// A checked switch submits `two_factor=true`; an unchecked one submits
/// nothing, read as `false`.
#[query_params]
#[derive(Clone, Debug, Default)]
#[serde(default)]
struct SecuritySettings {
    two_factor: bool,
}

fn validate_security_settings(settings: &SecuritySettings) -> Errors {
    let mut errors = Errors::new();
    if !settings.two_factor {
        errors.insert(
            "two_factor",
            "It is highly recommended to enable two-factor authentication.",
        );
    }
    errors
}

#[component]
pub async fn switch(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<SecuritySettings>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-switch"));
    let errors = submission
        .map(validate_security_settings)
        .unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let enabled = submission.is_some_and(|settings| settings.two_factor);
    let two_factor = FormField::new("form-switch-two-factor", errors.get("two_factor").copied());

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(
                card_title("Security Settings")
                card_description("Manage your account security preferences.")
            )
            card_content(
                <form
                    id="form-switch"
                    method="get"
                    action="#switch"
                    novalidate=""
                    class="grid gap-6"
                >
                    <input type="hidden" name="form" value="form-switch">
                    form_item(attrs: attributes! { class="flex! items-center justify-between gap-4!" },
                        <div class="grid gap-2">
                            form_label(field: two_factor, "Multi-factor authentication")
                            form_description(field: two_factor,
                                "Enable multi-factor authentication to secure your account."
                            )
                            form_message(field: two_factor)
                        </div>
                        ui::switch::switch(attrs: attributes! {
                            name="two_factor"
                            value="true"
                            checked=(enabled)
                            (two_factor.control(cx))
                        })
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2" },
                <a href="?#switch" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(attrs: attributes! { type="submit" form="form-switch" }, "Save")
            )
        )
    })
}

/// The plans of the complex form, as `(id, title, description)`.
const TIERS: [(&str, &str, &str); 2] = [
    ("basic", "Basic", "For individuals and small teams"),
    ("pro", "Pro", "For businesses with higher demands"),
];

/// The add-ons, as `(name, title, description)`.
const ADDONS: [(&str, &str, &str); 3] = [
    ("analytics", "Analytics", "Advanced analytics and reporting"),
    ("backup", "Backup", "Automated daily backups"),
    (
        "support",
        "Priority Support",
        "24/7 premium customer support",
    ),
];

#[query_params]
#[derive(Clone, Debug)]
#[serde(default)]
struct Subscription {
    plan: String,
    billing_period: String,
    analytics: bool,
    backup: bool,
    support: bool,
    email_notifications: bool,
}

/// The values the form starts with, which `#[serde(default)]` also uses for
/// keys a submission leaves out.
impl Default for Subscription {
    fn default() -> Self {
        Self {
            plan: "basic".to_owned(),
            billing_period: String::new(),
            analytics: false,
            backup: false,
            support: false,
            email_notifications: false,
        }
    }
}

impl Subscription {
    /// Whether the add-on named `name` is chosen.
    fn addon(&self, name: &str) -> bool {
        match name {
            "analytics" => self.analytics,
            "backup" => self.backup,
            "support" => self.support,
            _ => false,
        }
    }
}

fn validate_subscription(subscription: &Subscription) -> Errors {
    let mut errors = Errors::new();
    if !TIERS.iter().any(|(id, ..)| *id == subscription.plan) {
        errors.insert("plan", "Invalid plan selection. Please choose Basic or Pro");
    }
    if !matches!(subscription.billing_period.as_str(), "monthly" | "yearly") {
        errors.insert("billing_period", "Please select a billing period");
    }
    // One field per add-on, so there is no "too many" or "invalid add-on"
    // case to check.
    if !ADDONS.iter().any(|(name, ..)| subscription.addon(name)) {
        errors.insert("addons", "Please select at least one add-on");
    }
    errors
}

#[component]
pub async fn complex(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<Subscription>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-complex"));
    let errors = submission.map(validate_subscription).unwrap_or_default();
    let saved = submission.filter(|_| errors.is_empty());
    let values = submission.cloned().unwrap_or_default();
    let plan = FormField::new("form-complex-plan", errors.get("plan").copied());
    let billing_period = FormField::new(
        "form-complex-billing-period",
        errors.get("billing_period").copied(),
    );
    let addons = FormField::new("form-complex-addons", errors.get("addons").copied());
    let email_notifications = FormField::new("form-complex-email-notifications", None);

    Ok(view! {
        card(attrs: attributes! { class="w-full max-w-sm" },
            card_header(attrs: attributes! { class="border-b-2 border-border pb-6" },
                card_title("You're almost there!")
                card_description("Choose your subscription plan and billing period.")
            )
            card_content(
                <form
                    id="form-complex"
                    method="get"
                    action="#complex"
                    novalidate=""
                    class="grid gap-8"
                >
                    <input type="hidden" name="form" value="form-complex">
                    form_item(
                        form_label(field: plan, "Subscription Plan")
                        form_description(field: plan, "Choose your subscription plan.")
                        ui::radio_group::radio_group(
                            attrs: attributes! { class="gap-3" (plan.control(cx)) },
                            #[key(id)]
                            for (id, title, detail) in TIERS {
                                let item_id = format!("form-complex-plan-{id}");
                                let option = FormField::new(&item_id, plan.error);
                                form_item(
                                    attrs: attributes! {
                                        class="flex! items-start justify-between gap-4! rounded-base \
                                            border-2 border-border bg-secondary-background p-4"
                                    },
                                    <div class="grid gap-1.5">
                                        form_label(field: option, (title))
                                        form_description(field: option, (detail))
                                    </div>
                                    radio_group_item(attrs: attributes! {
                                        name="plan"
                                        value=(id)
                                        checked=(values.plan == id)
                                        (option.control(cx))
                                    })
                                )
                            }
                        )
                        form_message(field: plan)
                    )
                    form_item(
                        form_label(field: billing_period, "Billing Period")
                        ui::select::select(
                            placeholder: "Select",
                            attrs: attributes! {
                                name="billing_period"
                                required=""
                                (billing_period.control(cx))
                            },
                            select_content(
                                select_item(
                                    attrs: attributes! {
                                        value="monthly"
                                        selected=(values.billing_period == "monthly")
                                    },
                                    "Monthly"
                                )
                                select_item(
                                    attrs: attributes! {
                                        value="yearly"
                                        selected=(values.billing_period == "yearly")
                                    },
                                    "Yearly"
                                )
                            )
                        )
                        form_description(field: billing_period,
                            "Choose how often you want to be billed."
                        )
                        form_message(field: billing_period)
                    )
                    form_item(
                        form_label(field: addons, "Add-ons")
                        form_description(field: addons,
                            "Select additional features you'd like to include."
                        )
                        #[key(name)]
                        for (name, title, detail) in ADDONS {
                            let item_id = format!("form-complex-{name}");
                            let addon = FormField::new(&item_id, addons.error);
                            form_item(attrs: attributes! { class="flex! items-start gap-3!" },
                                ui::checkbox::checkbox(attrs: attributes! {
                                    name=(name)
                                    value="true"
                                    checked=(values.addon(name))
                                    (addon.control(cx))
                                })
                                <div class="grid gap-1.5">
                                    form_label(field: addon, (title))
                                    form_description(field: addon, (detail))
                                </div>
                            )
                        }
                        form_message(field: addons)
                    )
                    form_item(attrs: attributes! { class="flex! items-center justify-between gap-4!" },
                        <div class="grid gap-2">
                            form_label(field: email_notifications, "Email Notifications")
                            form_description(field: email_notifications,
                                "Receive email updates about your subscription"
                            )
                            form_message(field: email_notifications)
                        </div>
                        ui::switch::switch(attrs: attributes! {
                            name="email_notifications"
                            value="true"
                            checked=(values.email_notifications)
                            (email_notifications.control(cx))
                        })
                    )
                </form>
                if let Some(saved) = saved {
                    submitted_values(values: format!("{saved:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="border-t-2 border-border pt-6" },
                <div class="grid w-full gap-2">
                    button(attrs: attributes! { type="submit" form="form-complex" }, "Save Preferences")
                    <a href="?#complex" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                </div>
            )
        )
    })
}

/// The most addresses the array form takes.
const MAX_EMAILS: usize = 5;

/// The array form submits its addresses as `email-0`, `email-1`, ... A query
/// struct has one field per key, so `flatten` gathers the keys it does not
/// name into a map.
#[query_params]
#[derive(Debug, Default)]
#[serde(default)]
struct ContactEmails {
    /// The button that submitted the form: `add`, `remove-<index>` or `save`.
    intent: String,
    #[serde(flatten)]
    fields: HashMap<String, String>,
}

/// A light `name@domain.tld` check; the browser's `type="email"` check, when
/// the form is not `novalidate`, is stricter.
fn is_email(address: &str) -> bool {
    match address.split_once('@') {
        Some((name, domain)) => {
            !name.is_empty()
                && !address.contains(char::is_whitespace)
                && !domain.contains('@')
                && domain.split('.').count() > 1
                && domain.split('.').all(|part| !part.is_empty())
        }
        None => false,
    }
}

#[component]
pub async fn array(cx: &Cx) -> Result<impl View> {
    let submission = query_params::<ContactEmails>(cx)
        .ok()
        .filter(|_| submitted(cx, "form-array"));
    // The rows read back in order until an index is missing; a first visit
    // starts with two empty rows.
    let mut emails: Vec<String> = match submission {
        Some(submission) => (0..)
            .map_while(|index| submission.fields.get(&format!("email-{index}")).cloned())
            .collect(),
        None => vec![String::new(); 2],
    };
    // "Add" and "Remove" are submit buttons too: the server changes the rows
    // and renders the form again, without validating it.
    let intent = submission.map_or("", |submission| submission.intent.as_str());
    if intent == "add" && emails.len() < MAX_EMAILS {
        emails.push(String::new());
    }
    if let Some(index) = intent
        .strip_prefix("remove-")
        .and_then(|index| index.parse::<usize>().ok())
        && index < emails.len()
        && emails.len() > 1
    {
        emails.remove(index);
    }
    let saving = intent == "save";
    let row_errors: Vec<Option<&str>> = emails
        .iter()
        .map(|email| (saving && !is_email(email)).then_some("Enter a valid email address."))
        .collect();
    let list_error = if !saving {
        None
    } else if emails.is_empty() {
        Some("Add at least one email address.")
    } else if emails.len() > MAX_EMAILS {
        Some("You can add up to 5 email addresses.")
    } else {
        None
    };
    let saved = saving && list_error.is_none() && row_errors.iter().all(Option::is_none);
    let heading = FormField::new("form-array-emails", None);

    Ok(view! {
        card(attrs: attributes! { class="w-full sm:max-w-md" },
            card_header(attrs: attributes! { class="border-b-2 border-border pb-6" },
                card_title("Contact Emails")
                card_description("Manage your contact email addresses.")
            )
            card_content(
                <form
                    id="form-array"
                    method="get"
                    action="#array"
                    novalidate=""
                    class="grid gap-4"
                >
                    <input type="hidden" name="form" value="form-array">
                    // Enter in a field submits through the form's first submit
                    // button, which would otherwise be a row's "Remove".
                    <button
                        type="submit"
                        name="intent"
                        value="save"
                        class="sr-only"
                        tabindex="-1"
                        aria-hidden="true"
                    >
                        "Save"
                    </button>
                    <div class="grid gap-2">
                        form_label(field: heading, "Email Addresses")
                        <p class="text-sm font-base text-foreground">
                            "Add up to 5 email addresses where we can contact you."
                        </p>
                    </div>
                    #[key(index)]
                    for (index, email) in emails.iter().enumerate() {
                        let id = format!("form-array-email-{index}");
                        let address = FormField::new(&id, row_errors[index]);
                        form_item(
                            form_label(field: address, attrs: attributes! { class="sr-only" },
                                "Email " (index + 1)
                            )
                            input_group(
                                input_group_input(attrs: attributes! {
                                    name=(format!("email-{index}"))
                                    value=(email.as_str())
                                    placeholder="name@example.com"
                                    type="email"
                                    autocomplete="email"
                                    (address.control(cx))
                                })
                                if emails.len() > 1 {
                                    input_group_addon(align: InputGroupAddonAlign::InlineEnd,
                                        input_group_button(
                                            size: InputGroupButtonSize::IconXs,
                                            attrs: attributes! {
                                                type="submit"
                                                name="intent"
                                                value=(format!("remove-{index}"))
                                                aria-label=(format!("Remove email {}", index + 1))
                                            },
                                            icon(data: iconify_icon!("lucide:x"))
                                        )
                                    )
                                }
                            )
                            form_message(field: address)
                        )
                    }
                    button(
                        variant: ButtonVariant::Neutral,
                        size: ButtonSize::Sm,
                        attrs: attributes! {
                            type="submit"
                            name="intent"
                            value="add"
                            class="justify-self-start"
                            disabled=(emails.len() >= MAX_EMAILS)
                        },
                        "Add Email Address"
                    )
                    if let Some(error) = list_error {
                        <p class="text-sm font-base text-red-500">(error)</p>
                    }
                </form>
                if saved {
                    submitted_values(values: format!("{emails:#?}"))
                }
            )
            card_footer(attrs: attributes! { class="gap-2 border-t-2 border-border pt-6" },
                <a href="?#array" class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))>"Reset"</a>
                button(
                    attrs: attributes! { type="submit" form="form-array" name="intent" value="save" },
                    "Save"
                )
            )
        )
    })
}
