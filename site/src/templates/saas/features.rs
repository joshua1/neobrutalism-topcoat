//! A bento grid of feature cells, each with a small illustration.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{Child, View, attributes, class, component, view},
};

use super::{
    decor_star::{decor_star, section_star},
    section_heading::section_heading,
};
use crate::components::{
    avatar::{AvatarSize, avatar, avatar_fallback},
    kbd::{kbd, kbd_group},
    stars::Star,
};

/// Title, label, priority dot colour and owner's initials.
const ISSUES: [(&str, &str, &str, &str); 4] = [
    (
        "Checkout fails on Safari 17",
        "Bug · P1",
        "bg-chart-2",
        "SC",
    ),
    ("Add CSV export to reports", "Feature", "bg-chart-3", "JM"),
    (
        "Duplicate of PLN-88",
        "Duplicate",
        "bg-secondary-background",
        "AR",
    ),
    ("Webhook retries time out", "Bug · P2", "bg-chart-3", "TB"),
];

const TRIAGE_STATS: [(&str, &str); 3] = [
    ("24", "triaged today"),
    ("6", "duplicates closed"),
    ("0", "left in inbox"),
];

const SHORTCUTS: [(&[&str], &str); 3] = [
    (&["⌘", "K"], "Command menu"),
    (&["C"], "Create issue"),
    (&["G", "I"], "Go to inbox"),
];

// Icon data is a `static`, not a `const`.
static INTEGRATIONS: [(&str, IconData, &str); 6] = [
    ("GitHub", iconify_icon!("lucide:git-branch"), "bg-chart-5"),
    (
        "Slack",
        iconify_icon!("lucide:message-square"),
        "bg-chart-4",
    ),
    ("Stripe", iconify_icon!("lucide:credit-card"), "bg-chart-1"),
    ("Email", iconify_icon!("lucide:mail"), "bg-chart-3"),
    ("Zapier", iconify_icon!("lucide:zap"), "bg-chart-2"),
    ("Webhooks", iconify_icon!("lucide:webhook"), "bg-chart-4"),
];

static SECURITY: [(&str, IconData); 3] = [
    ("SOC 2 Type II", iconify_icon!("lucide:shield-check")),
    ("SAML SSO & SCIM", iconify_icon!("lucide:key-round")),
    ("Audit logs", iconify_icon!("lucide:scroll-text")),
];

/// Bar heights (percent) of the last ten sprints' cycle time.
const BARS: [u32; 10] = [62, 48, 70, 41, 55, 36, 44, 30, 38, 26];

/// A bento cell: title and description on top, the illustration (`child`)
/// pinned to the bottom. `decoration` sits behind the text.
#[component]
async fn cell(
    cx: &Cx,
    title: &str,
    description: &str,
    #[default] class: &str,
    #[default] decoration: Child<'_>,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(
            "relative flex flex-col overflow-hidden rounded-base border-2 border-border bg-secondary-background shadow-shadow",
            class,
        ))>
            (decoration)
            <div class="relative p-6 pb-0">
                <h3 class="font-heading text-xl">(title)</h3>
                <p class="mt-2 text-sm font-base">(description)</p>
            </div>
            <div class="relative flex flex-1 items-end p-6">(child)</div>
        </div>
    })
}

#[component]
pub async fn features(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section
            id="features"
            class="relative isolate scroll-mt-16 overflow-hidden border-b-4 border-border bg-background px-5 py-24"
        >
            section_star(
                shape: Star::S35,
                color: "var(--chart-2)",
                class: "-left-16 top-[62%] size-24 rotate-[10deg] lg:size-40",
            )
            section_star(
                shape: Star::S19,
                color: "var(--chart-3)",
                class: "-right-16 lg:-right-10 top-10 size-24 -rotate-[25deg] lg:size-40",
            )
            <div class="mx-auto w-full max-w-6xl">
                section_heading(
                    eyebrow: "Features",
                    title: "Everything your team needs. Nothing it doesn't.",
                    description: "From triage to release, Planly covers the whole loop.",
                )

                <div class="grid gap-5 md:grid-cols-6">
                    cell(
                        title: "AI triage that actually helps",
                        description: "Agents label, prioritise, dedupe and assign every incoming issue, so your inbox stays at zero.",
                        class: "md:col-span-6 lg:col-span-4 lg:row-span-2",
                        decoration: view! {
                            decor_star(
                                shape: Star::S25,
                                color: "var(--chart-5)",
                                attrs: attributes! {
                                    class="absolute -right-12 -top-12 hidden size-40 rotate-12 sm:block lg:hidden xl:block"
                                },
                            )
                        }
                        .into(),
                        <div class="w-full space-y-3">
                            <div class="mb-6 grid grid-cols-1 gap-3 sm:grid-cols-3">
                                for (value, label) in TRIAGE_STATS {
                                    <div class="rounded-base border-2 border-border bg-chart-3 p-2 shadow-shadow min-[375px]:p-3">
                                        <p class="font-heading text-2xl sm:text-3xl">(value)</p>
                                        <p class="hyphens-auto text-xs font-base wrap-anywhere">(label)</p>
                                    </div>
                                }
                            </div>
                            #[key(title)]
                            for (title, label, priority, owner) in ISSUES {
                                <div class="flex items-center gap-3 rounded-base border-2 border-border bg-background p-3">
                                    <span class=(class!("size-3 shrink-0 rounded-full border-2 border-border", priority))></span>
                                    <p class="flex-1 truncate text-sm font-heading">(title)</p>
                                    <span class="hidden items-center gap-1 rounded-base border-2 border-border bg-chart-3 px-1.5 text-[11px] font-heading sm:inline-flex">
                                        icon(data: iconify_icon!("lucide:sparkles"), attrs: attributes! { class="size-3" })
                                        (label)
                                    </span>
                                    // `!` beats the small avatar's `size-8` (no tailwind-merge here).
                                    avatar(
                                        size: AvatarSize::Sm,
                                        attrs: attributes! { class="size-7!" },
                                        avatar_fallback(
                                            attrs: attributes! { class="text-[10px] font-heading!" },
                                            (owner)
                                        )
                                    )
                                </div>
                            }
                        </div>
                    )

                    cell(
                        title: "Keyboard first",
                        description: "Every action is one shortcut away.",
                        class: "md:col-span-3 lg:col-span-2",
                        <div class="w-full space-y-2.5">
                            #[key(label)]
                            for (keys, label) in SHORTCUTS {
                                <div class="flex items-center justify-between rounded-base border-2 border-border bg-background px-3 py-2 text-sm font-base">
                                    (label)
                                    kbd_group(
                                        #[key(*key)]
                                        for key in keys {
                                            kbd((*key))
                                        }
                                    )
                                </div>
                            }
                        </div>
                    )

                    cell(
                        title: "Cycle analytics",
                        description: "See where work gets stuck before it slips.",
                        class: "md:col-span-3 lg:col-span-2",
                        <div class="w-full">
                            <p class="font-heading text-4xl">"−32%"</p>
                            <p class="text-xs font-base">"median cycle time, last 10 sprints"</p>
                            <div class="mt-4 flex h-20 items-end gap-1.5">
                                for (i, height) in BARS.into_iter().enumerate() {
                                    <div
                                        class=(class!(
                                            "flex-1 rounded-t-sm border-2 border-border",
                                            "bg-chart-4" if i == BARS.len() - 1 else "bg-chart-1",
                                        ))
                                        style=(format!("height: {height}%"))
                                    ></div>
                                }
                            </div>
                        </div>
                    )

                    cell(
                        title: "Plays well with your stack",
                        description: "Two-way sync with the tools you already use, plus an open API.",
                        class: "md:col-span-3",
                        <div class="grid w-full grid-cols-1 gap-3 min-[360px]:grid-cols-2 sm:grid-cols-3 md:grid-cols-2 lg:grid-cols-3">
                            for (name, data, color) in &INTEGRATIONS {
                                <div class="flex items-center gap-2 rounded-base border-2 border-border bg-background p-2.5 text-sm font-heading">
                                    <span class=(class!(
                                        "flex size-7 shrink-0 items-center justify-center rounded-base border-2 border-border",
                                        *color,
                                    ))>
                                        icon(data: data.clone(), attrs: attributes! { class="size-4" })
                                    </span>
                                    (*name)
                                </div>
                            }
                        </div>
                    )

                    cell(
                        title: "Enterprise-grade security",
                        description: "Your data is encrypted at rest and in transit, with controls your security team will love.",
                        // `!` beats the cell's `bg-secondary-background`.
                        class: "bg-chart-3! md:col-span-3",
                        <div class="flex w-full flex-col gap-3 sm:flex-row sm:items-end sm:justify-between">
                            <ul class="space-y-2">
                                for (label, data) in &SECURITY {
                                    <li class="flex items-center gap-2 text-sm font-heading">
                                        icon(data: data.clone(), attrs: attributes! { class="size-4" })
                                        " "
                                        (*label)
                                    </li>
                                }
                            </ul>
                            <div class="rounded-base border-2 border-border bg-secondary-background px-4 py-3 shadow-shadow">
                                <p class="font-heading text-3xl">"99.99%"</p>
                                <p class="text-xs font-base">"uptime SLA"</p>
                            </div>
                        </div>
                    )
                </div>
            </div>
        </section>
    })
}
