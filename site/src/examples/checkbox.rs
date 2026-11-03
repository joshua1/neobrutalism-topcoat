use topcoat::{
    Result,
    context::Cx,
    runtime::{Event, expr, signal},
    view::{View, attributes, component, view},
};

use crate::components::{
    checkbox::checkbox,
    field::{
        FieldLegendVariant, FieldOrientation, field, field_content, field_description,
        field_group, field_label, field_legend, field_set,
    },
    table::{table_body, table_cell, table_head, table_header, table_row},
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn basic() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="mx-auto w-56!" },
            field(orientation: FieldOrientation::Horizontal,
                checkbox(attrs: attributes! {
                    id="terms-checkbox-basic" name="terms-checkbox-basic"
                })
                field_label(attrs: attributes! { for="terms-checkbox-basic" },
                    "Accept terms and conditions"
                )
            )
        )
    })
}

#[component]
pub async fn description() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="mx-auto w-72!" },
            field(orientation: FieldOrientation::Horizontal,
                checkbox(attrs: attributes! {
                    id="terms-checkbox-desc" name="terms-checkbox-desc" checked=""
                })
                field_content(
                    field_label(attrs: attributes! { for="terms-checkbox-desc" },
                        "Accept terms and conditions"
                    )
                    field_description(
                        "By clicking this checkbox, you agree to the terms and conditions."
                    )
                )
            )
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="mx-auto w-56!" },
            field(
                orientation: FieldOrientation::Horizontal,
                attrs: attributes! { data-disabled="true" },
                checkbox(attrs: attributes! {
                    id="toggle-checkbox-disabled" name="toggle-checkbox-disabled" disabled=""
                })
                field_label(attrs: attributes! { for="toggle-checkbox-disabled" },
                    "Enable notifications"
                )
            )
        )
    })
}

#[component]
pub async fn group() -> Result<impl View> {
    let items = [
        ("finder-pref-hard-disks-checkbox", "Hard disks", true),
        ("finder-pref-external-disks-checkbox", "External disks", true),
        ("finder-pref-cds-dvds-checkbox", "CDs, DVDs, and iPods", false),
        ("finder-pref-connected-servers-checkbox", "Connected servers", false),
    ];
    Ok(view! {
        field_set(
            field_legend(variant: FieldLegendVariant::Label, "Show these items on the desktop:")
            field_description("Select the items you want to show on the desktop.")
            field_group(attrs: attributes! { class="gap-3!" },
                #[key(id)]
                for (id, text, checked) in items {
                    field(orientation: FieldOrientation::Horizontal,
                        checkbox(attrs: attributes! { id=(id) name=(id) checked=(checked) })
                        field_label(attrs: attributes! { for=(id) class="font-base!" }, (text))
                    )
                }
            )
        )
    })
}

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! { checkbox() })
}

#[component]
pub async fn invalid_state() -> Result<impl View> {
    Ok(view! {
        field_group(attrs: attributes! { class="mx-auto w-56!" },
            field(
                orientation: FieldOrientation::Horizontal,
                attrs: attributes! { data-invalid="true" },
                checkbox(attrs: attributes! {
                    id="terms-checkbox-invalid" name="terms-checkbox-invalid" aria-invalid="true"
                })
                field_label(attrs: attributes! { for="terms-checkbox-invalid" },
                    "Accept terms and conditions"
                )
            )
        )
    })
}

// `table` shares its name with the component, so that one is called by its
// full path.
#[component]
pub async fn table(cx: &Cx) -> Result<impl View> {
    // One signal per row: the runtime has no set type, so each row's
    // selection is its own `bool`.
    let sarah = signal(cx, || true);
    let marcus = signal(cx, || false);
    let priya = signal(cx, || false);
    let david = signal(cx, || false);

    let rows = [
        ("1", "Sarah Chen", "sarah.chen@example.com", "Admin", sarah.clone()),
        ("2", "Marcus Rodriguez", "marcus.rodriguez@example.com", "User", marcus.clone()),
        ("3", "Priya Patel", "priya.patel@example.com", "User", priya.clone()),
        ("4", "David Kim", "david.kim@example.com", "Editor", david.clone()),
    ];

    // Expressions have no `&&`, so "every row is selected" nests `if`s.
    let all_selected = expr!(if sarah.get() {
        if marcus.get() { if priya.get() { david.get() } else { false } } else { false }
    } else {
        false
    });

    Ok(view! {
        crate::components::table::table(
            table_header(
                table_row(
                    table_head(attrs: attributes! { class="w-8" },
                        checkbox(attrs: attributes! {
                            id="select-all-checkbox"
                            name="select-all-checkbox"
                            :checked=$(all_selected)
                            @change=$(|e: Event| {
                                let checked = e.target.checked;
                                sarah.set(checked);
                                marcus.set(checked);
                                priya.set(checked);
                                david.set(checked);
                            })
                        })
                    )
                    table_head("Name")
                    table_head("Email")
                    table_head("Role")
                )
            )
            table_body(
                #[key(id)]
                for (id, name, email, role, selected) in rows {
                    table_row(attrs: attributes! {
                        :data-state=$(selected.get().then_some("selected"))
                    },
                        table_cell(
                            checkbox(attrs: attributes! {
                                id=(format!("row-{id}-checkbox"))
                                name=(format!("row-{id}-checkbox"))
                                :checked=$(selected.get())
                                @change=$(|e: Event| selected.set(e.target.checked))
                            })
                        )
                        table_cell(attrs: attributes! { class="font-heading" }, (name))
                        table_cell((email))
                        table_cell((role))
                    )
                }
            )
        )
    })
}
