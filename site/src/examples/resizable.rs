use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::resizable::{
    ResizableOrientation, resizable_handle, resizable_panel, resizable_panel_group,
};

#[component]
pub async fn demo() -> Result<impl View> {
    // The inner group is vertical, so its handle is told so: it runs across
    // the group, as a horizontal line.
    Ok(view! {
        resizable_panel_group(
            attrs: attributes! {
                class="rounded-base max-w-md border-2 border-border text-foreground shadow-shadow"
            },
            resizable_panel(default_size: Some(50.0),
                <div class="flex h-[200px] items-center justify-center bg-background p-6">
                    <span class="font-base">"One"</span>
                </div>
            )
            resizable_handle()
            resizable_panel(default_size: Some(50.0),
                resizable_panel_group(orientation: ResizableOrientation::Vertical,
                    resizable_panel(default_size: Some(25.0),
                        <div class="flex h-full items-center justify-center bg-background p-6">
                            <span class="font-base">"Two"</span>
                        </div>
                    )
                    resizable_handle(orientation: ResizableOrientation::Vertical)
                    resizable_panel(default_size: Some(75.0),
                        <div class="flex h-full items-center justify-center bg-background p-6">
                            <span class="font-base">"Three"</span>
                        </div>
                    )
                )
            )
        )
    })
}
