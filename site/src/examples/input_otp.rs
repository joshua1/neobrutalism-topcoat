use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    field::{field, field_label},
    input_otp::{
        REGEXP_ONLY_DIGITS, REGEXP_ONLY_DIGITS_AND_CHARS, input_otp, input_otp_group,
        input_otp_separator, input_otp_slot,
    },
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn alphanumeric() -> Result<impl View> {
    Ok(view! {
        input_otp(max_length: 6, pattern: Some(REGEXP_ONLY_DIGITS_AND_CHARS),
            input_otp_group(
                input_otp_slot(index: 0)
                input_otp_slot(index: 1)
                input_otp_slot(index: 2)
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 3)
                input_otp_slot(index: 4)
                input_otp_slot(index: 5)
            )
        )
    })
}

#[component]
pub async fn controlled(cx: &Cx) -> Result<impl View> {
    let value = signal(cx, String::new);

    Ok(view! {
        <div class="space-y-2">
            input_otp(
                max_length: 6,
                attrs: attributes! { @input=$(|e: Event| value.set(e.target.value)) },
                input_otp_group(
                    #[key(index)]
                    for index in 0..6 {
                        input_otp_slot(index: index)
                    }
                )
            )
            <div class="text-center text-sm">
                $(if value.get().is_empty() {
                    ("Enter your one-time password.", "".to_owned())
                } else {
                    ("You entered: ", value.get())
                })
            </div>
        </div>
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        input_otp(max_length: 6, value: "123456", attrs: attributes! { id="disabled" disabled="" },
            input_otp_group(
                input_otp_slot(index: 0)
                input_otp_slot(index: 1)
                input_otp_slot(index: 2)
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 3)
                input_otp_slot(index: 4)
                input_otp_slot(index: 5)
            )
        )
    })
}

#[component]
pub async fn four_digits() -> Result<impl View> {
    Ok(view! {
        input_otp(max_length: 4, pattern: Some(REGEXP_ONLY_DIGITS),
            input_otp_group(
                input_otp_slot(index: 0)
                input_otp_slot(index: 1)
                input_otp_slot(index: 2)
                input_otp_slot(index: 3)
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        input_otp(max_length: 6,
            input_otp_group(
                input_otp_slot(index: 0)
                input_otp_slot(index: 1)
                input_otp_slot(index: 2)
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 3)
                input_otp_slot(index: 4)
                input_otp_slot(index: 5)
            )
        )
    })
}

#[component]
pub async fn invalid() -> Result<impl View> {
    let invalid = || attributes! { aria-invalid="true" };
    Ok(view! {
        input_otp(max_length: 6, value: "000000",
            input_otp_group(
                input_otp_slot(index: 0, attrs: invalid())
                input_otp_slot(index: 1, attrs: invalid())
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 2, attrs: invalid())
                input_otp_slot(index: 3, attrs: invalid())
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 4, attrs: invalid())
                input_otp_slot(index: 5, attrs: invalid())
            )
        )
    })
}

#[component]
pub async fn pattern() -> Result<impl View> {
    Ok(view! {
        field(attrs: attributes! { class="w-fit!" },
            field_label(attrs: attributes! { for="digits-only" }, "Digits Only")
            input_otp(
                max_length: 6,
                pattern: Some(REGEXP_ONLY_DIGITS),
                attrs: attributes! { id="digits-only" },
                input_otp_group(
                    #[key(index)]
                    for index in 0..6 {
                        input_otp_slot(index: index)
                    }
                )
            )
        )
    })
}

#[component]
pub async fn separator() -> Result<impl View> {
    Ok(view! {
        input_otp(max_length: 6,
            input_otp_group(
                input_otp_slot(index: 0)
                input_otp_slot(index: 1)
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 2)
                input_otp_slot(index: 3)
            )
            input_otp_separator()
            input_otp_group(
                input_otp_slot(index: 4)
                input_otp_slot(index: 5)
            )
        )
    })
}
