//! Two rows of quotes scrolling in opposite directions; hovering a row
//! pauses it.

use topcoat::{
    Result,
    context::Cx,
    view::{View, attributes, class, component, view},
};

use super::{
    decor_star::{decor_star, section_star},
    section_heading::section_heading,
};
use crate::components::{
    avatar::{avatar, avatar_fallback},
    stars::Star,
};

struct Testimonial {
    quote: &'static str,
    name: &'static str,
    role: &'static str,
    /// The avatar's background. `!` beats the avatar fallback's own.
    color: &'static str,
}

static TESTIMONIALS: [Testimonial; 8] = [
    Testimonial {
        quote: "We moved 40 engineers over from Jira in a single afternoon. Nobody has asked to go back.",
        name: "Alex Rivera",
        role: "VP Engineering, Acme",
        color: "bg-chart-3!",
    },
    Testimonial {
        quote: "The triage agent closes a third of our inbound bugs as duplicates before a human ever sees them.",
        name: "Sam Chen",
        role: "CTO, Globex",
        color: "bg-chart-1!",
    },
    Testimonial {
        quote: "It's the first tool where the keyboard shortcuts are so good that I stopped touching my mouse.",
        name: "Jordan Mills",
        role: "Staff Engineer, Hooli",
        color: "bg-chart-4!",
    },
    Testimonial {
        quote: "Roadmaps finally reflect reality. Leadership checks Planly instead of pinging me for updates.",
        name: "Taylor Brooks",
        role: "Head of Product, Initech",
        color: "bg-chart-5!",
    },
    Testimonial {
        quote: "Our cycle time dropped by a third in two months. The analytics made the bottlenecks obvious.",
        name: "Morgan Lee",
        role: "Engineering Manager, Umbrella",
        color: "bg-chart-2!",
    },
    Testimonial {
        quote: "SSO, SCIM and audit logs passed our security review on the first try. That never happens.",
        name: "Casey Walsh",
        role: "Security Lead, Cyberdyne",
        color: "bg-chart-3!",
    },
    Testimonial {
        quote: "Automations replaced three Zapier flows and a bot we maintained ourselves. Less glue, fewer pages.",
        name: "Riley Patel",
        role: "Platform Lead, Soylent",
        color: "bg-chart-1!",
    },
    Testimonial {
        quote: "Fast, clear and opinionated in the best way. Our designers actually enjoy filing issues now.",
        name: "Jamie Novak",
        role: "Design Director, Pied Piper",
        color: "bg-chart-4!",
    },
];

fn initials(name: &str) -> String {
    name.split(' ')
        .filter_map(|part| part.chars().next())
        .collect()
}

/// One copy of a row. Two copies scroll in step (the theme's `animate-marquee`
/// and `animate-marquee2`), so the loop has no gap.
#[component]
async fn track(
    cx: &Cx,
    items: &[&'static Testimonial],
    class: &str,
    #[default] reverse: bool,
    #[default] hidden: bool,
) -> Result<impl View> {
    let style = format!(
        "animation-duration: 70s; animation-direction: {}",
        if reverse { "reverse" } else { "normal" }
    );
    Ok(view! {
        <div
            aria-hidden=(hidden.then_some("true"))
            class=(class!(
                "flex shrink-0 py-3 group-hover:[animation-play-state:paused] motion-reduce:animate-none",
                class,
            ))
            style=(style)
        >
            #[key(t.name)]
            for t in items {
                <figure class="mx-1.5 flex w-64 shrink-0 flex-col justify-between gap-4 rounded-base border-2 border-border bg-secondary-background p-4 shadow-shadow sm:mx-2.5 sm:w-85 sm:gap-6 sm:p-6">
                    <div class="flex flex-col gap-3 sm:gap-4">
                        <div class="flex gap-1" role="img" aria-label="5 out of 5 stars">
                            for _ in 0..5 {
                                decor_star(
                                    shape: Star::S4,
                                    stroke_width: 1.25,
                                    attrs: attributes! { class="size-4 sm:size-5" },
                                )
                            }
                        </div>
                        <blockquote class="text-sm font-base leading-relaxed sm:text-base">
                            "“" (t.quote) "”"
                        </blockquote>
                    </div>
                    <figcaption class="flex items-center gap-3">
                        // `!` beats the avatar's default `size-10`.
                        avatar(attrs: attributes! { class="size-8! sm:size-10!" },
                            avatar_fallback(
                                attrs: attributes! { class=(class!(t.color, "text-xs font-heading! sm:text-sm")) },
                                (initials(t.name))
                            )
                        )
                        <div>
                            <p class="text-sm font-heading">(t.name)</p>
                            <p class="text-xs font-base">(t.role)</p>
                        </div>
                    </figcaption>
                </figure>
            }
        </div>
    })
}

#[component]
async fn row(
    cx: &Cx,
    items: &[&'static Testimonial],
    #[default] reverse: bool,
) -> Result<impl View> {
    Ok(view! {
        <div class="group relative flex overflow-hidden">
            track(items: items, reverse: reverse, class: "animate-marquee")
            track(items: items, reverse: reverse, class: "absolute left-0 top-0 animate-marquee2", hidden: true)
        </div>
    })
}

#[component]
pub async fn testimonials(cx: &Cx) -> Result<impl View> {
    let forward: Vec<&'static Testimonial> = TESTIMONIALS.iter().collect();
    let backward: Vec<&'static Testimonial> = TESTIMONIALS.iter().rev().collect();

    Ok(view! {
        <section class="relative isolate overflow-hidden border-b-4 border-border bg-background py-24">
            section_star(
                shape: Star::S30,
                color: "var(--chart-2)",
                class: "-right-16 lg:-right-12 top-6 size-24 rotate-[40deg] lg:size-40",
            )
            section_star(
                shape: Star::S22,
                color: "var(--chart-3)",
                class: "-left-16 lg:-left-12 bottom-10 size-24 -rotate-[10deg] lg:size-40",
            )
            section_heading(
                eyebrow: "Testimonials",
                title: "Teams can't stop talking about it",
                description: "Rated 4.9/5 by 2,000+ teams.",
                class: "px-5",
            )

            <div class="space-y-2">
                row(items: &forward)
                row(items: &backward, reverse: true)
            </div>
        </section>
    })
}
