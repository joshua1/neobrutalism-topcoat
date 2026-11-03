//! The Octane examples use `Input` and `Label`, which are ported in another
//! batch; they are plain elements with the same classes here.

use topcoat::{
    Result,
    view::{View, attributes, class, component, view},
};

use crate::components::button::{ButtonSize, ButtonVariant, button, button_variants};
use crate::components::sheet::{
    SheetSide, sheet, sheet_close, sheet_content, sheet_description, sheet_footer, sheet_header,
    sheet_title, sheet_trigger,
};

/// `Label` classes, from `label.tsrx`.
const LABEL: &str = "text-sm font-heading leading-none peer-disabled:cursor-not-allowed \
    peer-disabled:opacity-70";

/// `Input` classes, from `input.tsrx`.
const INPUT: &str = "flex h-10 w-full rounded-base border-2 border-border bg-secondary-background \
    selection:bg-main selection:text-main-foreground px-3 py-2 text-sm font-base text-foreground \
    placeholder:text-foreground/50 focus-visible:outline-hidden focus-visible:ring-2 \
    focus-visible:ring-black focus-visible:ring-offset-2 disabled:cursor-not-allowed \
    disabled:opacity-50";

const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod \
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
    exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor \
    in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est \
    laborum.";

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        sheet_trigger(
            target: "sheet-demo",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Default, ButtonSize::Default))
            },
            "Open"
        )
        sheet(
            id: "sheet-demo",
            attrs: attributes! { aria-labelledby="sheet-demo-title" },
            sheet_content(
                sheet_header(
                    sheet_title(attrs: attributes! { id="sheet-demo-title" }, "Edit profile")
                    sheet_description(
                        "Make changes to your profile here. Click save when you're done."
                    )
                )
                <div class="grid flex-1 auto-rows-min gap-6 px-4">
                    <div class="grid gap-3">
                        <label for="sheet-demo-name" class=(LABEL)>"Name"</label>
                        <input id="sheet-demo-name" value="Pedro Duarte" class=(INPUT)>
                    </div>
                    <div class="grid gap-3">
                        <label for="sheet-demo-username" class=(LABEL)>"Username"</label>
                        <input id="sheet-demo-username" value="@octanejs" class=(INPUT)>
                    </div>
                </div>
                sheet_footer(
                    button(attrs: attributes! { type="submit" }, "Save changes")
                    sheet_close(
                        attrs: attributes! {
                            class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
                        },
                        "Close"
                    )
                )
            )
        )
    })
}

#[component]
pub async fn no_close_button() -> Result<impl View> {
    Ok(view! {
        sheet_trigger(
            target: "sheet-no-close-button",
            attrs: attributes! {
                class=(button_variants(ButtonVariant::Neutral, ButtonSize::Default))
            },
            "Open Sheet"
        )
        sheet(
            id: "sheet-no-close-button",
            attrs: attributes! { aria-labelledby="sheet-no-close-button-title" },
            sheet_content(show_close_button: false,
                sheet_header(
                    sheet_title(
                        attrs: attributes! { id="sheet-no-close-button-title" },
                        "No Close Button"
                    )
                    sheet_description(
                        "This sheet doesn't have a close button in the top-right corner. \
                         Click outside to close."
                    )
                )
            )
        )
    })
}

/// Each side's label (for the trigger and the ids) and value.
const SHEET_SIDES: [(&str, SheetSide); 4] = [
    ("top", SheetSide::Top),
    ("right", SheetSide::Right),
    ("bottom", SheetSide::Bottom),
    ("left", SheetSide::Left),
];

#[component]
pub async fn side() -> Result<impl View> {
    Ok(view! {
        <div class="flex gap-2">
            for (name, side) in SHEET_SIDES {
                let id = format!("sheet-side-{name}");
                let title_id = format!("{id}-title");

                sheet_trigger(
                    target: &id,
                    attrs: attributes! {
                        class=(class!(
                            button_variants(ButtonVariant::NoShadow, ButtonSize::Default),
                            "capitalize",
                        ))
                    },
                    (name)
                )
                sheet(
                    id: id.as_str(),
                    attrs: attributes! { aria-labelledby=(&title_id) },
                    sheet_content(side: side,
                        sheet_header(
                            sheet_title(attrs: attributes! { id=(&title_id) }, "Edit profile")
                            sheet_description(
                                "Make changes to your profile here. Click save when you're done."
                            )
                        )
                        <div class="overflow-y-auto px-4 text-sm">
                            <h4 class="mb-4 text-lg leading-none font-medium">"Lorem Ipsum"</h4>
                            for _ in 0..10 {
                                <p class="mb-4 leading-normal">(LOREM)</p>
                            }
                        </div>
                        sheet_footer(
                            button(attrs: attributes! { type="submit" }, "Save changes")
                            sheet_close(
                                attrs: attributes! {
                                    class=(button_variants(
                                        ButtonVariant::Neutral,
                                        ButtonSize::Default,
                                    ))
                                },
                                "Cancel"
                            )
                        )
                    )
                )
            }
        </div>
    })
}
