//! Questions in an accordion, beside a card to contact a human.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use super::{
    decor_star::{GRID_BACKGROUND, decor_star},
    section_heading::{Align, section_heading},
};
use crate::components::{
    accordion::{accordion, accordion_content, accordion_item, accordion_trigger},
    button::{ButtonVariant, button},
    stars::Star,
};

const FAQS: [(&str, &str); 5] = [
    (
        "How long does it take to switch from Jira or GitHub Issues?",
        "Most teams are fully moved in an afternoon. The importer brings over issues, comments, labels, assignees and history, and you can run both tools side by side while you switch.",
    ),
    (
        "Is there really a free plan?",
        "Yes. Starter is free forever for up to 10 members with unlimited issues. You only pay when you need AI triage, roadmaps or more seats.",
    ),
    (
        "What do the AI agents have access to?",
        "Agents only read the workspaces you enable them for, and every action they take is logged and reversible. Your data is never used to train shared models.",
    ),
    (
        "Can I change plans or cancel at any time?",
        "You can upgrade, downgrade or cancel whenever you like. Changes are prorated to the day and you can export all of your data at any point.",
    ),
    (
        "Do you offer discounts for startups and non-profits?",
        "Startups under two years old get 50% off Pro for the first year, and registered non-profits get Pro for free. Reach out to our team to apply.",
    ),
];

#[component]
pub async fn faq(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section
            id="faq"
            class=(class!(
                "scroll-mt-16 border-b-4 border-border bg-secondary-background px-5 py-24",
                GRID_BACKGROUND,
            ))
        >
            // Below lg the left column dissolves into the grid, so the contact
            // card can move below the accordion.
            <div class="mx-auto grid w-full max-w-6xl gap-8 lg:grid-cols-[1fr_1.5fr] lg:gap-12">
                <div class="contents lg:flex lg:flex-col">
                    section_heading(
                        eyebrow: "FAQ",
                        title: "Questions? Answers.",
                        description: "Everything you need to know about Planly. Can't find what you're looking for?",
                        align: Align::Left,
                        class: "mb-4! lg:mb-8!",
                    )
                    <div class="relative order-last overflow-hidden lg:order-0 rounded-base border-2 border-border bg-background p-6 shadow-shadow">
                        decor_star(
                            shape: Star::S22,
                            color: "var(--chart-2)",
                            attrs: attributes! { class="absolute -bottom-10 -right-10 size-32 rotate-12" },
                        )
                        icon(data: iconify_icon!("lucide:message-circle"), attrs: attributes! { class="size-6" })
                        <p class="mt-3 font-heading text-lg">"Talk to a human"</p>
                        <p class="mt-1 text-sm font-base">"Our team replies in under 2 hours on weekdays."</p>
                        button(variant: ButtonVariant::Neutral, attrs: attributes! { class="mt-5" }, "Chat with us")
                    </div>
                </div>

                // One shared `name` lets one answer open at a time.
                accordion(attrs: attributes! { class="space-y-4" },
                    for (question, answer) in FAQS {
                        accordion_item(attrs: attributes! { name="saas-faq" },
                            accordion_trigger((question))
                            accordion_content(attrs: attributes! { class="text-base" }, (answer))
                        )
                    }
                )
            </div>
        </section>
    })
}
