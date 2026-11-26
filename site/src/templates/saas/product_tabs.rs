//! Three product views behind tabs that rotate on their own (with a progress
//! bar) until the visitor picks one. Each view responds to the pointer.

use topcoat::{
    Result,
    context::Cx,
    icon::{IconData, icon, iconify::iconify_icon},
    runtime::{Event, expr, signal},
    view::{Child, Unescaped, View, attributes, class, component, view},
};

use super::{
    decor_star::{GRID_BACKGROUND, section_star},
    section_heading::section_heading,
};
use crate::components::{
    stars::Star,
    switch::switch,
    tabs::{tabs, tabs_content, tabs_list, tabs_trigger},
};

struct Tab {
    value: &'static str,
    label: &'static str,
    blurb: &'static str,
    icon: IconData,
    color: &'static str,
}

// Icon data is a `static`, not a `const`.
static TABS: [Tab; 3] = [
    Tab {
        value: "roadmap",
        label: "Roadmap",
        blurb: "Every project on one timeline, from idea to launch.",
        icon: iconify_icon!("lucide:map"),
        color: "bg-chart-3",
    },
    Tab {
        value: "analytics",
        label: "Analytics",
        blurb: "Cycle time and throughput, no spreadsheet required.",
        icon: iconify_icon!("lucide:chart-column"),
        color: "bg-chart-4",
    },
    Tab {
        value: "automations",
        label: "Automations",
        blurb: "Rules that handle triage, handoffs and reminders.",
        icon: iconify_icon!("lucide:workflow"),
        color: "bg-chart-2",
    },
];

const MONTHS: [&str; 6] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"];

struct RoadmapItem {
    name: &'static str,
    /// The month it starts in, from 0.
    start: usize,
    /// How many months it spans.
    span: usize,
    color: &'static str,
    status: &'static str,
    progress: u32,
    owner: &'static str,
}

const ROADMAP: [RoadmapItem; 5] = [
    RoadmapItem {
        name: "Public API v2",
        start: 0,
        span: 2,
        color: "bg-chart-1",
        status: "Shipped",
        progress: 100,
        owner: "Platform",
    },
    RoadmapItem {
        name: "Usage billing",
        start: 1,
        span: 3,
        color: "bg-chart-4",
        status: "In progress",
        progress: 64,
        owner: "Billing",
    },
    RoadmapItem {
        name: "Mobile app",
        start: 2,
        span: 3,
        color: "bg-chart-3",
        status: "In progress",
        progress: 38,
        owner: "Mobile",
    },
    RoadmapItem {
        name: "SSO & SCIM",
        start: 4,
        span: 2,
        color: "bg-chart-2",
        status: "Planned",
        progress: 0,
        owner: "Security",
    },
    RoadmapItem {
        name: "Insights 2.0",
        start: 3,
        span: 3,
        color: "bg-chart-5",
        status: "In progress",
        progress: 21,
        owner: "Data",
    },
];

/// Label, value and change of the analytics stats.
const STATS: [(&str, &str, &str); 3] = [
    ("Issues closed", "1,284", "+18%"),
    ("Avg. cycle time", "2.4d", "−32%"),
    ("Sprint scope hit", "94%", "+7%"),
];

/// Issues closed and opened (percent of the chart's height) per week.
const THROUGHPUT: [(u32, u32); 8] = [
    (40, 22),
    (52, 18),
    (46, 30),
    (61, 20),
    (58, 26),
    (72, 15),
    (66, 24),
    (80, 12),
];

struct Rule {
    when: &'static str,
    condition: &'static str,
    then: &'static str,
    enabled: bool,
}

const RULES: [Rule; 3] = [
    Rule {
        when: "Issue created",
        condition: "label is Bug",
        then: "Assign to on-call",
        enabled: true,
    },
    Rule {
        when: "PR merged",
        condition: "linked to an issue",
        then: "Move issue to Done",
        enabled: true,
    },
    Rule {
        when: "Issue stale",
        condition: "no activity for 14 days",
        then: "Ping the owner in Slack",
        enabled: false,
    },
];

const CHIP: &str =
    "rounded-base border-2 border-border px-2 py-1 text-xs font-heading transition-colors";

/// Rotates the tabs every 6 seconds while at least 40% of them is on screen,
/// filling the active trigger's progress bar meanwhile. The first tab the
/// visitor picks stops the rotation, and so does a preference for reduced
/// motion. The tabs element is the script's previous sibling; switching
/// itself is the tabs component's own click handler.
const AUTOPLAY_SCRIPT: &str = r#"(() => {
  const root = document.currentScript.previousElementSibling;
  const triggers = [...root.querySelectorAll("[data-slot=tabs-trigger]")];
  const bars = triggers.map((trigger) => trigger.querySelector("[data-bar]"));
  let animation = null;
  let inView = false;
  const stop = () => {
    animation?.cancel();
    animation = null;
    for (const bar of bars) bar.hidden = true;
  };
  if (matchMedia("(prefers-reduced-motion: reduce)").matches) return stop();
  const start = () => {
    const index = triggers.findIndex((trigger) => trigger.hasAttribute("data-active"));
    bars.forEach((bar, i) => (bar.hidden = i !== index));
    animation = bars[index].firstElementChild.animate(
      [{ transform: "scaleX(0)" }, { transform: "scaleX(1)" }],
      { duration: 6000, fill: "both" },
    );
    if (!inView) animation.pause();
    animation.onfinish = () => {
      triggers[(index + 1) % triggers.length].click();
      start();
    };
  };
  // Capture, so this runs before the trigger selects itself.
  root.addEventListener("click", (e) => {
    const trigger = e.target.closest("[data-slot=tabs-trigger]");
    if (animation && e.isTrusted && trigger && !trigger.hasAttribute("data-active")) stop();
  }, true);
  new IntersectionObserver(([entry]) => {
    inView = entry.isIntersecting;
    if (animation) inView ? animation.play() : animation.pause();
  }, { threshold: 0.4 }).observe(root);
  start();
})();"#;

/// An app window: traffic lights and a title bar over the content.
#[component]
async fn window_frame(cx: &Cx, title: Child<'_>, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="flex h-full flex-col overflow-hidden rounded-base border-2 border-border bg-secondary-background shadow-shadow">
            <div class="flex items-center gap-1.5 border-b-2 border-border bg-background px-4 py-2.5">
                <span class="size-3 rounded-full border-2 border-border bg-chart-2"></span>
                <span class="size-3 rounded-full border-2 border-border bg-chart-3"></span>
                <span class="size-3 rounded-full border-2 border-border bg-chart-4"></span>
                <span class="ml-2 text-xs font-heading">(title)</span>
            </div>
            <div class="flex flex-1 flex-col p-5 sm:p-8">(child)</div>
        </div>
    })
}

#[component]
async fn chip(cx: &Cx, class: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <span class=(class!(CHIP, class))>(child)</span>
    })
}

/// A timeline; pointing at a bar shows its project below.
#[component]
async fn roadmap_view(cx: &Cx) -> Result<impl View> {
    let selected = signal(cx, || 1usize);
    // Snapshots of the rows for the browser. Owned strings, because the
    // runtime reads `[String; N]` but not `[&str; N]` elements.
    let names = ROADMAP.map(|item| item.name.to_owned());
    let colors = ROADMAP.map(|item| item.color.to_owned());
    let statuses = ROADMAP.map(|item| item.status.to_owned());
    let owners = ROADMAP.map(|item| item.owner.to_owned());
    let progress = ROADMAP.map(|item| format!("{}%", item.progress));
    let widths = ROADMAP.map(|item| format!("width: {}%", item.progress));

    Ok(view! {
        window_frame(title: view! { "Roadmap · H1 2026" }.into(),
            <div class="mb-6 grid grid-cols-[72px_repeat(6,minmax(0,1fr))] gap-y-3 sm:grid-cols-[140px_repeat(6,minmax(0,1fr))]">
                <span></span>
                for month in MONTHS {
                    <span class="border-l-2 border-dashed border-border pl-1 text-[11px] font-heading sm:pl-2 sm:text-xs">
                        (month)
                    </span>
                }
                #[key(i)]
                for (i, item) in ROADMAP.iter().enumerate() {
                    let color = item.color;
                    <div class="contents">
                        <span class="col-start-1 self-center pr-2 text-xs leading-tight font-heading sm:truncate sm:pr-3 sm:text-sm">
                            (item.name)
                        </span>
                        <button
                            type="button"
                            @click=$(|_e: Event| selected.set(i))
                            @pointerenter=$(|_e: Event| selected.set(i))
                            @focus=$(|_e: Event| selected.set(i))
                            :aria-pressed=$(if selected.get() == i { "true" } else { "false" })
                            aria-label=(format!("{}: {}", item.name, item.status))
                            :class=$((
                                "relative h-9 overflow-hidden rounded-base border-2 border-border text-left transition-all ",
                                color,
                                if selected.get() == i {
                                    " -translate-x-0.5 -translate-y-0.5 shadow-[4px_4px_0_0_var(--border)]"
                                } else {
                                    " shadow-[2px_2px_0_0_var(--border)]"
                                },
                            ))
                            style=(format!("grid-column: {} / span {}", item.start + 2, item.span))
                        >
                            <span
                                class="absolute inset-y-0 left-0 border-r-2 border-border bg-[repeating-linear-gradient(45deg,transparent_0_6px,#00000026_6px_8px)]"
                                style=(format!(
                                    "width: {}%; border-right-width: {}px",
                                    item.progress,
                                    if item.progress % 100 == 0 { 0 } else { 2 },
                                ))
                            ></span>
                        </button>
                    </div>
                }
            </div>

            <div
                aria-live="polite"
                class="mt-auto flex flex-wrap items-center gap-x-4 gap-y-2 rounded-base border-2 border-border bg-background px-4 py-3 text-sm font-base"
            >
                <span :class=$(("size-4 rounded-sm border-2 border-border ", colors[selected.get()].to_owned()))></span>
                <span class="font-heading">$(names[selected.get()].to_owned())</span>
                chip(class: "bg-secondary-background", $(statuses[selected.get()].to_owned()))
                <span>"Owner: " $(owners[selected.get()].to_owned())</span>
                <span class="flex items-center gap-2 sm:ml-auto">
                    <span class="h-3 w-24 overflow-hidden rounded-full border-2 border-border bg-secondary-background">
                        <span
                            class="block h-full bg-foreground transition-[width] duration-500"
                            :style=$(widths[selected.get()].to_owned())
                        ></span>
                    </span>
                    <span class="font-heading">$(progress[selected.get()].to_owned())</span>
                </span>
            </div>
        )
    })
}

/// Stats and a weekly throughput chart; pointing at a week shows its numbers.
#[component]
async fn analytics_view(cx: &Cx) -> Result<impl View> {
    let week = signal(cx, || THROUGHPUT.len() - 1);
    // Snapshots of the numbers for the browser, as strings: an expression
    // can't return a borrowed array element, and strings can be copied out
    // with `to_owned`.
    let closed = THROUGHPUT.map(|(closed, _)| closed.to_string());
    let opened = THROUGHPUT.map(|(_, opened)| opened.to_string());

    Ok(view! {
        window_frame(title: view! { "Analytics · Last 8 weeks" }.into(),
            <div class="grid gap-4 sm:grid-cols-3">
                for (label, value, delta) in STATS {
                    <div class="rounded-base border-2 border-border bg-background p-4">
                        <p class="text-xs font-base">(label)</p>
                        <div class="mt-1 flex items-baseline justify-between">
                            <p class="font-heading text-3xl">(value)</p>
                            chip(class: "bg-chart-4", (delta))
                        </div>
                    </div>
                }
            </div>

            <div class="mt-6 flex flex-wrap items-center justify-between gap-3 text-xs font-base">
                <div class="flex gap-5">
                    <span class="flex items-center gap-1.5">
                        <span class="size-3 rounded-sm border-2 border-border bg-chart-1"></span>
                        "Closed"
                    </span>
                    <span class="flex items-center gap-1.5">
                        <span class="size-3 rounded-sm border-2 border-border bg-chart-3"></span>
                        "Opened"
                    </span>
                </div>
                <p aria-live="polite" class="font-heading">
                    "Week " $(week.get() + 1)
                    " · " $(closed[week.get()].to_owned())
                    " closed · " $(opened[week.get()].to_owned()) " opened"
                </p>
            </div>

            <div class="mt-3 flex h-48 items-end gap-2 sm:gap-4 lg:h-auto lg:min-h-48 lg:flex-1">
                #[key(i)]
                for (i, (closed, opened)) in THROUGHPUT.into_iter().enumerate() {
                    <button
                        type="button"
                        @click=$(|_e: Event| week.set(i))
                        @pointerenter=$(|_e: Event| week.set(i))
                        @focus=$(|_e: Event| week.set(i))
                        :aria-pressed=$(if week.get() == i { "true" } else { "false" })
                        aria-label=(format!("Week {}: {closed} closed, {opened} opened", i + 1))
                        :class=$((
                            "-mx-1 -mb-1.5 flex h-[calc(100%+0.375rem)] flex-1 flex-col justify-end rounded-base border-2 px-1 pb-1 pt-1 transition-colors sm:-mx-1.5 sm:-mb-2 sm:h-[calc(100%+0.5rem)] sm:px-1.5 sm:pb-1.5 ",
                            if week.get() == i { "border-dashed border-border bg-background" } else { "border-transparent" },
                        ))
                    >
                        <span
                            class="block rounded-t-base border-2 border-b-0 border-border bg-chart-3 transition-[height] duration-500"
                            style=(format!("height: {opened}%"))
                        ></span>
                        <span
                            class="block border-2 border-border bg-chart-1 transition-[height] duration-500"
                            style=(format!("height: {closed}%"))
                        ></span>
                    </button>
                }
            </div>
            <div class="mt-3.5 flex gap-2 text-center text-[11px] font-heading sm:gap-4">
                for i in 1..=THROUGHPUT.len() {
                    <span class="flex-1">(format!("W{i}"))</span>
                }
            </div>
        )
    })
}

/// Automation rules, each with a switch.
#[component]
async fn automations_view(cx: &Cx) -> Result<impl View> {
    let first = signal(cx, || RULES[0].enabled);
    let second = signal(cx, || RULES[1].enabled);
    let third = signal(cx, || RULES[2].enabled);
    let active = expr!(
        (if first.get() { 1usize } else { 0usize })
            + (if second.get() { 1usize } else { 0usize })
            + (if third.get() { 1usize } else { 0usize })
    );
    let rules = [(&RULES[0], first), (&RULES[1], second), (&RULES[2], third)];

    Ok(view! {
        window_frame(title: view! { "Automations · " $(active) (format!(" of {} active", RULES.len())) }.into(),
            <div class="space-y-4">
                #[key(rule.when)]
                for (rule, on) in rules {
                    <div :class=$((
                        "flex flex-col gap-4 rounded-base border-2 border-border p-4 transition-all sm:flex-row sm:items-center sm:justify-between ",
                        if on.get() {
                            "bg-background shadow-[2px_2px_0_0_var(--border)]"
                        } else {
                            "border-dashed bg-secondary-background"
                        },
                    ))>
                        <div class="flex flex-wrap items-center gap-2 text-sm font-base">
                            <span class="font-heading">"When"</span>
                            <span :class=$((CHIP, if on.get() { " bg-chart-1" } else { " bg-background" }))>(rule.when)</span>
                            <span class="font-heading">"and"</span>
                            <span :class=$((CHIP, if on.get() { " bg-chart-3" } else { " bg-background" }))>(rule.condition)</span>
                            icon(data: iconify_icon!("lucide:arrow-right"), attrs: attributes! { class="size-4" })
                            <span :class=$((CHIP, if on.get() { " bg-chart-4" } else { " bg-background" }))>(rule.then)</span>
                        </div>
                        <div class="flex items-center gap-3 self-end sm:self-auto">
                            <span class="w-8 text-right text-xs font-heading">$(if on.get() { "On" } else { "Off" })</span>
                            switch(attrs: attributes! {
                                checked=(rule.enabled)
                                aria-label=(format!("Toggle rule: {}", rule.when))
                                @change=$(|e: Event| on.set(e.target.checked))
                            })
                        </div>
                    </div>
                }
            </div>
        )
    })
}

/// Overrides for the tabs component's trigger. `class!` doesn't merge
/// conflicting classes, so the ones the trigger already sets carry `!`.
const TRIGGER: &str = "relative h-full flex-col items-start! justify-start! gap-2! overflow-hidden \
     whitespace-normal! border-border! bg-background p-2.5! pb-4! text-left shadow-shadow \
     sm:gap-3! sm:p-5! sm:pb-6! lg:justify-center! lg:gap-2! lg:p-4! lg:pb-5! \
     not-data-active:hover:-translate-x-0.5 not-data-active:hover:-translate-y-0.5 \
     not-data-active:hover:shadow-[6px_6px_0_0_var(--border)]";

/// The panels slide in when shown, like the original's `animate-in`.
const PANEL: &str = "mt-0! lg:col-start-2 lg:h-full transition-[opacity,translate] duration-300 \
     starting:translate-y-3 starting:opacity-0";

#[component]
pub async fn product_tabs(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section
            id="product"
            class=(class!(
                "relative isolate scroll-mt-16 overflow-hidden border-b-4 border-border bg-secondary-background px-5 py-24",
                GRID_BACKGROUND,
            ))
        >
            section_star(
                shape: Star::S24,
                color: "var(--chart-4)",
                class: "-right-16 lg:-right-12 bottom-6 size-24 rotate-[35deg] lg:size-40",
            )
            section_star(
                shape: Star::S11,
                color: "var(--chart-5)",
                class: "-left-16 lg:-left-8 top-[18%] size-24 -rotate-6 lg:size-40",
            )

            <div class="relative mx-auto w-full max-w-6xl">
                section_heading(
                    eyebrow: "Product",
                    title: "One workspace. Every view.",
                    description: "Switch between roadmap, analytics and automations without losing context.",
                )

                tabs(
                    attrs: attributes! {
                        class="lg:grid lg:grid-cols-[minmax(0,320px)_minmax(0,1fr)] lg:min-h-120 lg:items-stretch lg:gap-8"
                    },
                    tabs_list(
                        attrs: attributes! {
                            class="mb-8 grid! h-auto! w-full grid-cols-1 gap-3 sm:grid-cols-3 border-0! bg-transparent! p-0! sm:gap-5 lg:mb-0 lg:h-full! lg:grid-cols-1 lg:grid-rows-3"
                        },
                        #[key(tab.value)]
                        for (i, tab) in TABS.iter().enumerate() {
                            tabs_trigger(
                                value: tab.value,
                                active: i == 0,
                                attrs: attributes! { class=(TRIGGER) },
                                <span class="flex w-full min-w-0 flex-row items-center gap-3 sm:flex-col sm:items-start lg:flex-row lg:items-center">
                                    <span class=(class!(
                                        "flex size-9 shrink-0 items-center justify-center rounded-base border-2 border-border text-foreground sm:size-10 lg:size-9",
                                        tab.color,
                                    ))>
                                        icon(data: tab.icon.clone(), attrs: attributes! { class="size-4 sm:size-5 lg:size-4" })
                                    </span>
                                    <span class="max-w-full hyphens-auto wrap-anywhere text-base sm:text-xl lg:text-lg">
                                        (tab.label)
                                    </span>
                                </span>
                                <span class="hidden text-sm font-base sm:block">(tab.blurb)</span>
                                // The autoplay's progress, shown on the active trigger.
                                <span
                                    data-bar=""
                                    hidden=(i != 0)
                                    class="absolute inset-x-0 bottom-0 h-2 border-t-2 border-border bg-secondary-background"
                                >
                                    <span class="block h-full origin-left bg-foreground" style="transform: scaleX(0)"></span>
                                </span>
                            )
                        }
                    )

                    tabs_content(value: "roadmap", active: true, attrs: attributes! { class=(PANEL) }, roadmap_view())
                    tabs_content(value: "analytics", attrs: attributes! { class=(PANEL) }, analytics_view())
                    tabs_content(value: "automations", attrs: attributes! { class=(PANEL) }, automations_view())
                )
                <script>(Unescaped::new_unchecked(AUTOPLAY_SCRIPT))</script>
            </div>
        </section>
    })
}
