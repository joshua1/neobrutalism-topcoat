use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::native_select::{
    native_select, native_select_opt_group, native_select_option,
};

// `class!` appends without merging, so a class that replaces one the
// component already sets carries Tailwind's `!` (important) modifier.

#[component]
pub async fn demo() -> Result<impl View> {
    Ok(view! {
        native_select(attrs: attributes! { class="w-full! max-w-xs" },
            native_select_option(attrs: attributes! { value="" }, "Select a fruit")
            native_select_option(attrs: attributes! { value="apple" }, "Apple")
            native_select_option(attrs: attributes! { value="banana" }, "Banana")
            native_select_option(attrs: attributes! { value="blueberry" }, "Blueberry")
            native_select_option(attrs: attributes! { value="grapes" disabled="" }, "Grapes")
            native_select_option(attrs: attributes! { value="pineapple" }, "Pineapple")
        )
    })
}

#[component]
pub async fn disabled() -> Result<impl View> {
    Ok(view! {
        native_select(attrs: attributes! { class="w-full! max-w-xs" disabled="" },
            native_select_option(attrs: attributes! { value="" }, "Disabled")
            native_select_option(attrs: attributes! { value="apple" }, "Apple")
            native_select_option(attrs: attributes! { value="banana" }, "Banana")
            native_select_option(attrs: attributes! { value="blueberry" }, "Blueberry")
        )
    })
}

#[component]
pub async fn with_optgroup() -> Result<impl View> {
    Ok(view! {
        native_select(attrs: attributes! { class="w-full! max-w-xs" },
            native_select_option(attrs: attributes! { value="" }, "Select a food")
            native_select_opt_group(attrs: attributes! { label="Fruits" },
                native_select_option(attrs: attributes! { value="apple" }, "Apple")
                native_select_option(attrs: attributes! { value="banana" }, "Banana")
                native_select_option(attrs: attributes! { value="blueberry" }, "Blueberry")
            )
            native_select_opt_group(attrs: attributes! { label="Vegetables" },
                native_select_option(attrs: attributes! { value="carrot" }, "Carrot")
                native_select_option(attrs: attributes! { value="broccoli" }, "Broccoli")
                native_select_option(attrs: attributes! { value="spinach" }, "Spinach")
            )
        )
    })
}
