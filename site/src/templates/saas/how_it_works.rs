//! Three steps joined by a line.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{View, attributes, class, component, view},
};

use super::{decor_star::section_star, section_heading::section_heading};
use crate::components::stars::Star;

/// Icon, title, description and icon tile colour.
static STEPS: [(IconData, &str, &str, &str); 3] = [
    (
        iconify_icon!("lucide:file-input"),
        "Import in one click",
        "Bring issues, projects and people over from Jira, GitHub or a CSV. History and comments come along too.",
        "bg-chart-1",
    ),
    (
        iconify_icon!("lucide:workflow"),
        "Automate the busywork",
        "Turn on agents and rules that triage, label and assign, so humans only touch what needs a human.",
        "bg-chart-3",
    ),
    (
        iconify_icon!("lucide:rocket"),
        "Ship on schedule",
        "Plan cycles, spot blockers early and release on time, every time. Your stakeholders get live updates.",
        "bg-chart-4",
    ),
];

#[component]
pub async fn how_it_works(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class="relative isolate overflow-hidden border-b-4 border-border bg-background px-5 py-24">
            section_star(
                shape: Star::S29,
                color: "var(--chart-3)",
                class: "-left-16 lg:-left-10 top-[40%] size-24 rotate-[50deg] lg:size-40",
            )
            section_star(
                shape: Star::S4,
                color: "var(--chart-2)",
                class: "-right-16 lg:-right-10 top-4 size-24 -rotate-12 lg:size-40",
            )

            <div class="relative mx-auto w-full max-w-6xl">
                section_heading(
                    eyebrow: "How it works",
                    title: "Up and running before your coffee cools",
                    description: "Most teams switch in an afternoon. No consultants, no migration project.",
                )

                <ol class="relative grid gap-12 md:grid-cols-3 md:gap-8">
                    <span
                        aria-hidden="true"
                        class="absolute left-[16.67%] right-[16.67%] top-6 hidden -translate-y-1/2 border-t-2 border-border md:block"
                    ></span>
                    for (data, title, description, color) in &STEPS {
                        <li class="relative flex flex-col items-center text-center">
                            <span class=(class!(
                                "flex size-12 items-center justify-center rounded-base border-2 border-border",
                                *color,
                            ))>
                                icon(data: data.clone(), attrs: attributes! { class="size-6" })
                            </span>
                            <h3 class="mt-6 font-heading text-xl">(*title)</h3>
                            <p class="mt-2 max-w-xs text-sm font-base">(*description)</p>
                        </li>
                    }
                </ol>
            </div>
        </section>
    })
}
