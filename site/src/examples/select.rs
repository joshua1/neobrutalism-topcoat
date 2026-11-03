use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

use crate::components::select::{
    select, select_content, select_group, select_item, select_label, select_trigger, select_value,
};

/// The fruits the demos list: value and label.
const FRUITS: [(&str, &str); 5] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("blueberry", "Blueberry"),
    ("grapes", "Grapes"),
    ("pineapple", "Pineapple"),
];

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        select(
            placeholder: "Select a fruit",
            attrs: attributes! { name="fruit" class="w-[180px]" },
            select_content(
                select_group(attrs: attributes! { label="Fruits" },
                    select_label("Fruits")
                    #[key(value)]
                    for (value, label) in FRUITS {
                        select_item(attrs: attributes! { value=(value) }, (label))
                    }
                )
            )
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        select(
            placeholder: "Disabled",
            attrs: attributes! { name="fruit" class="w-[180px]" disabled="" },
            select_content(
                #[key(value)]
                for (value, label) in FRUITS {
                    select_item(
                        attrs: attributes! { value=(value) disabled=(value == "grapes") },
                        (label)
                    )
                }
            )
        )
    })
}

#[component]
pub async fn large_list() -> Result<impl View> {
    Ok(view! {
        select(
            placeholder: "Large List",
            attrs: attributes! { name="item" class="w-[180px]" },
            select_content(
                #[key(i)]
                for i in 0..100 {
                    select_item(
                        attrs: attributes! { value=(format!("item-{i}")) },
                        (format!("Item {i}"))
                    )
                }
            )
        )
    })
}

/// Options with icons. `select_trigger` and `select_value` let browsers with
/// customizable selects show the chosen icon in the closed control too; the
/// placeholder is a hand-written option so it can hold an icon.
#[component]
pub async fn with_icon() -> Result<impl View> {
    Ok(view! {
        select(attrs: attributes! { name="chart" class="w-[180px]" },
            select_trigger(select_value())
            <option value="" disabled="" selected="" hidden="">
                icon(data: iconify_icon!("lucide:circle-dashed"))
                "With Icon"
            </option>
            select_content(
                select_item(attrs: attributes! { value="line" },
                    icon(data: iconify_icon!("lucide:chart-line"))
                    "Line"
                )
                select_item(attrs: attributes! { value="bar" },
                    icon(data: iconify_icon!("lucide:chart-bar"))
                    "Bar"
                )
                select_item(attrs: attributes! { value="pie" },
                    icon(data: iconify_icon!("lucide:chart-pie"))
                    "Pie"
                )
            )
        )
    })
}
