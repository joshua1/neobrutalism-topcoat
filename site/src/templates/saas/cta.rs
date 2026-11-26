//! The closing call to action: one big star in waves of smaller ones.

use std::f64::consts::PI;

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use super::email_capture::email_capture;
use crate::components::{
    button::ButtonVariant,
    stars::{Star, star},
};

const PERKS: [&str; 3] = [
    "14-day free trial",
    "No credit card required",
    "Cancel anytime",
];

// Main blue (chart-1) is left out so the centre star stays the only
// interactive-looking one.
const SHAPES: [Star; 4] = [Star::S15, Star::S22, Star::S26, Star::S30];
const COLORS: [&str; 4] = [
    "var(--chart-2)",
    "var(--chart-3)",
    "var(--chart-4)",
    "var(--chart-5)",
];

// Concentric waves that cover the whole background. Units are percent of the
// main star's width, so the waves scale with it. Each wave is slightly smaller
// than the one inside it and overlaps it heavily; the star count comes from
// the circumference so neighbours always overlap too. The first wave tucks
// behind the main star's rays, and waves continue until they pass the edges
// of a 4K screen. Each wave is offset half a step from the previous one so
// the stars interleave.
const FIRST_RADIUS: f64 = 46.0;
const FIRST_SIZE: f64 = 24.0;
/// Each wave's stars relative to the previous wave.
const SHRINK: f64 = 0.965;
/// Radial spacing, as a share of the two waves' average size.
const STEP: f64 = 0.45;
/// Arc between neighbours, as a share of star size.
const OVERLAP: f64 = 0.85;
/// Waves reach past the edges of a 3840px (4K) screen: half its width is
/// about 2.5 times the 780px main star.
const MAX_RADIUS: f64 = 255.0;
/// The section is the main star's height minus 200px, so only a horizontal
/// band is ever visible (about 34% of the star's width above and below centre
/// on mobile, 37% on desktop). Stars entirely outside it are skipped; the
/// margin covers rotated corners.
const VISIBLE_HALF_HEIGHT: f64 = 38.0;
/// Stars added to each wave on top of the computed count.
const EXTRA_PER_WAVE: usize = 2;

/// The seeded random generator of the original (mulberry32), so the waves
/// come out star for star the same.
struct Random(i32);

impl Random {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_add(0x6d2b_79f5);
        let seed = self.0;
        let mut t = (seed ^ ((seed as u32 >> 15) as i32)).wrapping_mul(1 | seed);
        t = t.wrapping_add((t ^ ((t as u32 >> 7) as i32)).wrapping_mul(61 | t)) ^ t;
        f64::from((t ^ ((t as u32 >> 14) as i32)) as u32) / 4_294_967_296.0
    }

    /// An index below `len` that differs from `avoid`. Like the original, an
    /// `avoid` of -1 (a wave's first star) never gives 0.
    fn pick_other(&mut self, len: usize, avoid: i32) -> i32 {
        let n = (self.next() * (len - 1) as f64).floor() as i32;
        if n >= avoid { n + 1 } else { n }
    }
}

/// JavaScript's `Math.round(n * 100) / 100`: halves round up.
fn round2(n: f64) -> f64 {
    let scaled = n * 100.0;
    let floor = scaled.floor();
    (if scaled - floor >= 0.5 {
        floor + 1.0
    } else {
        floor
    }) / 100.0
}

struct WaveStar {
    shape: i32,
    color: &'static str,
    x: f64,
    y: f64,
    size: f64,
    transform: String,
}

fn count_for(radius: f64, size: f64) -> usize {
    ((2.0 * PI * radius) / (size * OVERLAP)).round() as usize + EXTRA_PER_WAVE
}

/// Every visible star of every wave, outer waves first so inner waves overlap
/// them, stacking towards the centre. Positions follow the circles exactly;
/// only the shape and colour of each star are shuffled, never repeating the
/// previous star's in the same wave.
fn wave_stars() -> Vec<WaveStar> {
    let mut rings = Vec::new();
    let (mut radius, mut size) = (FIRST_RADIUS, FIRST_SIZE);
    while radius < MAX_RADIUS {
        rings.push((count_for(radius, size), radius, size));
        let next_size = size * SHRINK;
        radius += ((size + next_size) / 2.0) * STEP;
        size = next_size;
    }

    let mut random = Random(7);
    let mut stars = Vec::new();
    for (ring, (count, radius, size)) in rings.into_iter().enumerate() {
        let (mut shape, mut color) = (-1, -1);
        for i in 0..count {
            shape = random.pick_other(SHAPES.len(), shape);
            color = random.pick_other(COLORS.len(), color);
            let angle = ((i as f64 + (ring % 2) as f64 / 2.0) / count as f64) * 2.0 * PI - PI / 2.0;
            let x = 50.0 + radius * angle.cos();
            let y = 50.0 + radius * angle.sin();
            // Half-diagonal, so a star rotated 45° is still counted at full reach.
            if (y - 50.0).abs() - size * 0.71 > VISIBLE_HALF_HEIGHT {
                continue;
            }
            stars.push(WaveStar {
                shape,
                color: COLORS[color as usize],
                x: round2(x - size / 2.0),
                y: round2(y - size / 2.0),
                size: round2(size),
                transform: format!(
                    "rotate({} {} {})",
                    (i * 37 + ring * 19) % 90,
                    round2(x),
                    round2(y)
                ),
            });
        }
    }
    stars.reverse();
    stars
}

/// Hundreds of stars, so each shape is defined once and reused with `<use>`.
#[component]
async fn star_waves(cx: &Cx) -> Result<impl View> {
    let stars = wave_stars();
    Ok(view! {
        <svg
            aria-hidden="true"
            viewBox="0 0 100 100"
            class="pointer-events-none absolute inset-0 size-full overflow-visible"
        >
            <defs>
                for (i, shape) in SHAPES.into_iter().enumerate() {
                    <symbol id=(format!("cta-wave-star-{i}")) viewBox="0 0 200 200">
                        star(
                            shape: shape,
                            color: "currentColor",
                            stroke: "var(--border)",
                            stroke_width: 2.0,
                            path_class: "[vector-effect:non-scaling-stroke]",
                            attrs: attributes! { class="overflow-visible" },
                        )
                    </symbol>
                }
            </defs>
            for wave in stars {
                <use
                    href=(format!("#cta-wave-star-{}", wave.shape))
                    x=(wave.x.to_string())
                    y=(wave.y.to_string())
                    width=(wave.size.to_string())
                    height=(wave.size.to_string())
                    transform=(wave.transform)
                    style=(format!("color: {}", wave.color))
                ></use>
            }
        </svg>
    })
}

#[component]
pub async fn cta(cx: &Cx) -> Result<impl View> {
    Ok(view! {
        <section class="overflow-hidden border-b-4 border-border bg-background px-5">
            // The section is the main star's height minus 200px, so the star
            // and the waves are clipped top and bottom. The content sits
            // inside the main star's solid centre (about 68% of its width,
            // before the rays start), well clear of the clipped edges.
            <div class="relative mx-auto flex min-h-105 w-full max-w-6xl items-center justify-center sm:min-h-145 md:min-h-100 lg:min-h-145">
                <div class="absolute left-1/2 top-1/2 size-155 -translate-x-1/2 -translate-y-1/2 sm:size-195">
                    star_waves()
                    star(
                        shape: Star::S30,
                        color: "var(--main)",
                        stroke: "var(--border)",
                        stroke_width: 2.0,
                        path_class: "[vector-effect:non-scaling-stroke]",
                        attrs: attributes! {
                            aria-hidden="true"
                            class="pointer-events-none relative size-full overflow-visible"
                        },
                    )
                </div>

                <div class="relative flex w-full max-w-75 flex-col items-center text-center text-main-foreground sm:max-w-105">
                    <h2 class="font-heading text-3xl leading-tight sm:text-4xl">
                        "Your backlog won't triage itself."
                        <br>
                        "Well, now it will."
                    </h2>
                    <p class="mt-3 text-sm font-base sm:mt-4 sm:max-w-xs sm:text-base">
                        "Join 2,000+ teams who plan less and ship more with Planly."
                    </p>

                    <div class="mt-5 flex w-full flex-col items-center gap-3 sm:mt-6">
                        // `flex-row!` beats the form's own `flex-col`.
                        email_capture(cta: "Get started", variant: ButtonVariant::Neutral, class: "mx-auto flex-row!")
                        <ul class="flex flex-wrap justify-center gap-x-3 gap-y-1 text-xs font-base">
                            for perk in PERKS {
                                <li class="flex items-center gap-1">
                                    icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="size-3.5" })
                                    " "
                                    (perk)
                                </li>
                            }
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    })
}
