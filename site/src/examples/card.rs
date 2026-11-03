use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    badge::{BadgeVariant, badge},
    button::{ButtonSize, ButtonVariant, button},
    card::{
        CardSize, card, card_action, card_content, card_description, card_footer, card_header,
        card_title,
    },
    input::input,
    label::label,
    toggle::{ToggleKind, ToggleSize},
    toggle_group::{toggle_group, toggle_group_item},
};

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! { class="w-full max-w-sm" },
            card_header(
                card_title("Login to your account")
                card_description("Enter your email below to login to your account")
            )
            card_content(
                <form>
                    <div class="flex flex-col gap-6">
                        <div class="grid gap-2">
                            label(attrs: attributes! { for="email" }, "Email")
                            input(attrs: attributes! {
                                id="email"
                                type="email"
                                placeholder="m@example.com"
                                required=""
                            })
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center">
                                label(attrs: attributes! { for="password" }, "Password")
                                <a
                                    href="#"
                                    class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                                >
                                    "Forgot your password?"
                                </a>
                            </div>
                            input(attrs: attributes! { id="password" type="password" required="" })
                        </div>
                    </div>
                </form>
            )
            card_footer(
                attrs: attributes! { class="flex-col gap-2" },
                button(attrs: attributes! { type="submit" class="w-full" }, "Login")
                button(
                    variant: ButtonVariant::Neutral,
                    attrs: attributes! { class="w-full" },
                    "Login with Google"
                )
                <div class="mt-4 text-center text-sm">
                    "Don't have an account? "
                    <a href="#" class="underline underline-offset-4">"Sign up"</a>
                </div>
            )
        )
    })
}

#[component]
pub async fn image() -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! { class="relative mx-auto w-full max-w-sm overflow-hidden pt-0" },
            <div class="absolute inset-0 z-30 aspect-video bg-black/35"></div>
            <img
                src="https://avatar.vercel.sh/shadcn1"
                alt="Event cover"
                class="relative z-20 aspect-video w-full object-cover brightness-60 grayscale dark:brightness-40"
            >
            card_header(
                card_action(badge(variant: BadgeVariant::Neutral, "Featured"))
                card_title("Design systems meetup")
                card_description(
                    "A practical talk on component APIs, accessibility, and shipping faster."
                )
            )
            card_footer(button(attrs: attributes! { class="w-full" }, "View Event"))
        )
    })
}

#[component]
pub async fn size() -> Result<impl View> {
    let feature_name = "Scheduled reports";
    let points = [
        "Choose a schedule (daily, or weekly).",
        "Send to channels or specific teammates.",
        "Include charts, tables, and key metrics.",
    ];

    Ok(view! {
        card(
            size: CardSize::Sm,
            attrs: attributes! { class="mx-auto w-full max-w-xs" },
            card_header(
                card_title((feature_name))
                card_description("Weekly snapshots. No more manual exports.")
            )
            card_content(
                <ul class="grid gap-2 py-2 text-sm">
                    for point in points {
                        <li class="flex gap-2">
                            icon(
                                data: iconify_icon!("lucide:chevron-right"),
                                attrs: attributes! { class="mt-0.5 size-4 shrink-0 text-foreground" }
                            )
                            <span>(point)</span>
                        </li>
                    }
                </ul>
            )
            card_footer(
                attrs: attributes! { class="flex-col gap-2" },
                button(size: ButtonSize::Sm, attrs: attributes! { class="w-full" },
                    "Set up scheduled reports"
                )
                button(
                    variant: ButtonVariant::Neutral,
                    size: ButtonSize::Sm,
                    attrs: attributes! { class="w-full" },
                    "See what's new"
                )
            )
        )
    })
}

/// The spacing choices: the button label and the `--spacing(..)` multiple.
const SPACING_OPTIONS: [(&str, &str); 4] = [("16px", "4"), ("20px", "5"), ("24px", "6"), ("32px", "8")];

#[component]
pub async fn spacing(cx: &Cx) -> Result<impl View> {
    let spacing = signal(cx, || "4".to_owned());

    Ok(view! {
        <div class="mx-auto grid w-full max-w-sm gap-4">
            toggle_group(
                attrs: attributes! { class="justify-center" },
                #[key(value)]
                for (label_text, value) in SPACING_OPTIONS {
                    toggle_group_item(
                        kind: ToggleKind::Exclusive,
                        size: ToggleSize::Sm,
                        attrs: attributes! {
                            name="card-spacing"
                            value=(value)
                            checked=(value == "4")
                            @change=$(|e: Event| spacing.set(e.target.value))
                        },
                        (label_text)
                    )
                }
            )
            // An inline style beats the card's own `--card-spacing` class.
            card(
                attrs: attributes! {
                    :style=$(("--card-spacing: calc(var(--spacing) * ", spacing.get(), ")"))
                },
                card_header(
                    card_title("Login to your account")
                    card_description("Enter your email below to login to your account")
                    card_action(
                        <a href="#" class="text-sm underline-offset-4 hover:underline">"Sign Up"</a>
                    )
                )
                card_content(
                    <form>
                        <div class="flex flex-col gap-6">
                            <div class="grid gap-2">
                                label(attrs: attributes! { for="email-spacing" }, "Email")
                                input(attrs: attributes! {
                                    id="email-spacing"
                                    type="email"
                                    placeholder="m@example.com"
                                    required=""
                                })
                            </div>
                            <div class="grid gap-2">
                                <div class="flex items-center">
                                    label(attrs: attributes! { for="password-spacing" }, "Password")
                                    <a
                                        href="#"
                                        class="ml-auto inline-block text-sm underline-offset-4 hover:underline"
                                    >
                                        "Forgot your password?"
                                    </a>
                                </div>
                                input(attrs: attributes! { id="password-spacing" type="password" required="" })
                            </div>
                        </div>
                    </form>
                )
                card_footer(
                    attrs: attributes! { class="flex-col gap-2" },
                    button(attrs: attributes! { type="submit" class="w-full" }, "Login")
                    button(
                        variant: ButtonVariant::Neutral,
                        attrs: attributes! { class="w-full" },
                        "Login with Google"
                    )
                )
            )
        </div>
    })
}
