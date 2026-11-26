//! Three plans with a monthly/yearly billing switch.

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, class, component, view},
};

use super::{
    decor_star::{GRID_BACKGROUND, section_star, star_sticker},
    section_heading::section_heading,
};
use crate::components::{
    badge::badge,
    button::{ButtonVariant, button},
    card::{card, card_content, card_description, card_footer, card_header, card_title},
    stars::Star,
    switch::switch,
};

struct Plan {
    name: &'static str,
    description: &'static str,
    /// Dollars per user per month, billed monthly.
    monthly: u32,
    /// Dollars per user per month, billed yearly.
    yearly: u32,
    cta: &'static str,
    popular: bool,
    features: &'static [&'static str],
}

const PLANS: [Plan; 3] = [
    Plan {
        name: "Starter",
        description: "For small teams getting organised.",
        monthly: 0,
        yearly: 0,
        cta: "Start for free",
        popular: false,
        features: &[
            "Up to 10 members",
            "Unlimited issues",
            "2 active cycles",
            "GitHub & Slack integrations",
        ],
    },
    Plan {
        name: "Pro",
        description: "For growing teams that ship every week.",
        monthly: 12,
        yearly: 10,
        cta: "Start 14-day trial",
        popular: true,
        features: &[
            "Unlimited members",
            "AI triage agents",
            "Roadmaps & cycle analytics",
            "Automations (100 rules)",
            "Priority support",
        ],
    },
    Plan {
        name: "Business",
        description: "For companies with security and scale needs.",
        monthly: 24,
        yearly: 19,
        cta: "Contact sales",
        popular: false,
        features: &[
            "Everything in Pro",
            "SAML SSO & SCIM",
            "Audit logs & data residency",
            "Unlimited automations",
            "Dedicated success manager",
        ],
    },
];

#[component]
pub async fn pricing(cx: &Cx) -> Result<impl View> {
    // Billed yearly; the switch flips every price in the browser.
    let yearly = signal(cx, || true);

    Ok(view! {
        <section
            id="pricing"
            class=(class!(
                "relative isolate scroll-mt-16 overflow-hidden border-b-4 border-border bg-secondary-background px-5 py-24",
                GRID_BACKGROUND,
            ))
        >
            section_star(
                shape: Star::S15,
                color: "var(--chart-5)",
                class: "-left-16 lg:-left-10 bottom-8 size-24 -rotate-[15deg] lg:size-40",
            )
            section_star(
                shape: Star::S6,
                color: "var(--chart-4)",
                class: "-right-16 top-[35%] size-24 rotate-[22deg] lg:size-40",
            )
            <div class="mx-auto w-full max-w-6xl">
                section_heading(
                    eyebrow: "Pricing",
                    title: "Pricing that scales with you",
                    description: "Start free. Upgrade when your team does.",
                    class: "mb-8!",
                )

                <div class="mb-14 flex justify-center">
                    <div class="inline-flex items-center gap-3 rounded-base border-2 border-border bg-background px-4 py-2 text-sm shadow-shadow">
                        <span :class=$(if yearly.get() { "font-base" } else { "font-heading" })>"Monthly"</span>
                        switch(attrs: attributes! {
                            checked=""
                            aria-label="Bill yearly"
                            @change=$(|e: Event| yearly.set(e.target.checked))
                        })
                        <span :class=$(if yearly.get() { "font-heading" } else { "font-base" })>"Yearly"</span>
                        // `!` beats the badge's own background and weight.
                        badge(attrs: attributes! { class="bg-chart-4! font-heading!" }, "Save 20%")
                    </div>
                </div>

                <div class="grid items-start gap-6 md:grid-cols-3 md:gap-4 lg:gap-6">
                    #[key(plan.name)]
                    for plan in PLANS {
                        let (monthly, yearly_price) = (plan.monthly, plan.yearly);
                        let billed_yearly = format!("${} billed yearly", plan.yearly * 12);
                        card(attrs: attributes! { class="relative" },
                            if plan.popular {
                                star_sticker(
                                    shape: Star::S16,
                                    color: "var(--chart-4)",
                                    class: "-right-4 -top-10 text-xs",
                                    "Most"
                                    <br>
                                    "popular"
                                )
                            }

                            card_header(
                                // In the original, tailwind-merge drops the title's `leading-none` for
                                // `text-2xl`'s own line height; `leading-8!` restores that here.
                                card_title(attrs: attributes! { class="text-2xl leading-8!" }, (plan.name))
                                card_description((plan.description))
                                <div class="mt-4 flex items-baseline gap-1">
                                    <span class="font-heading text-5xl">
                                        "$" $(if yearly.get() { yearly_price } else { monthly })
                                    </span>
                                    <span class="text-sm font-base">"/user/mo"</span>
                                </div>
                                <p class="text-xs font-base">
                                    if plan.monthly == 0 {
                                        "Free forever"
                                    } else {
                                        $(if yearly.get() { billed_yearly.to_owned() } else { "Billed monthly".to_owned() })
                                    }
                                </p>
                            )

                            card_footer(
                                button(
                                    variant: if plan.popular { ButtonVariant::Default } else { ButtonVariant::Neutral },
                                    attrs: attributes! { class="w-full" },
                                    (plan.cta)
                                )
                            )

                            card_content(attrs: attributes! { class="border-t-2 border-border pt-6" },
                                <ul class="space-y-3">
                                    for feature in plan.features {
                                        <li class="flex items-center gap-2.5 text-sm font-base">
                                            <span class=(class!(
                                                "flex size-5 shrink-0 items-center justify-center rounded-base border-2 border-border",
                                                "bg-chart-3" if plan.popular else "bg-secondary-background",
                                            ))>
                                                icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="size-3" })
                                            </span>
                                            (*feature)
                                        </li>
                                    }
                                </ul>
                            )
                        )
                    }
                </div>
            </div>
        </section>
    })
}
