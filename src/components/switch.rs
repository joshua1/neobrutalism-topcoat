// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    view::{Attributes, StaticClass, View, class, component, view},
};

/// The size of a [`switch`].
///
/// [`Default`] is `SwitchSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SwitchSize {
    /// The standard track and thumb.
    #[default]
    Default,
    /// A smaller track and thumb.
    Sm,
}

impl SwitchSize {
    /// The `data-size` value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
        }
    }

    /// Classes for the track's dimensions.
    fn root_classes(self) -> StaticClass {
        match self {
            Self::Default => class!("h-6 w-12"),
            Self::Sm => class!("h-5 w-9"),
        }
    }

    /// Classes for the thumb's dimensions and its checked position.
    fn thumb_classes(self) -> StaticClass {
        match self {
            Self::Default => class!("h-4 w-4 peer-checked:translate-x-6"),
            Self::Sm => class!("h-3 w-3 peer-checked:translate-x-4"),
        }
    }
}

/// Classes for the track, the `peer` that a following label reads.
///
/// Base UI's state selectors read the native input inside instead:
/// `data-checked:` becomes `has-checked:`, `data-unchecked:` becomes
/// `not-has-checked:`, `data-disabled:` becomes `has-disabled:`, and
/// `focus-visible:` becomes `has-focus-visible:`.
const ROOT: StaticClass = class!(
    "peer relative inline-flex shrink-0 cursor-pointer items-center rounded-full border-2 \
     border-border bg-secondary-background transition-colors \
     has-focus-visible:outline-none has-focus-visible:ring-2 has-focus-visible:ring-ring \
     has-focus-visible:ring-offset-2 has-disabled:cursor-not-allowed has-disabled:opacity-50 \
     has-checked:bg-main not-has-checked:bg-secondary-background",
);

/// Classes for the native checkbox: invisible, covering the track so it
/// takes the clicks, the focus and the form value.
const INPUT: StaticClass = class!(
    "peer absolute inset-0 m-0 size-full cursor-[inherit] appearance-none rounded-full opacity-0",
);

/// Classes for the thumb, slid by the input's checked state.
const THUMB: StaticClass = class!(
    "pointer-events-none block rounded-full bg-white border-2 border-border ring-0 \
     transition-transform peer-not-checked:translate-x-1",
);

/// A neobrutalism on/off switch built on a native checkbox with the
/// `switch` role.
///
/// `size` defaults to `Default`. Every attribute and handler in `attrs` goes
/// on the `<input>`: `id`, `name`, `checked=""` for the initial state
/// (Octane's `defaultChecked`), `disabled`, `aria-invalid="true"`, and
/// `@change` to react (`e.target.checked`). Classes in `attrs` go on the
/// track instead.
///
/// ```ignore
/// view! {
///     <div class="flex items-center gap-2">
///         switch(attrs: attributes! { id="airplane-mode" })
///         label(attrs: attributes! { for="airplane-mode" }, "Airplane Mode")
///     </div>
/// }
/// ```
#[component]
pub async fn switch(
    #[default] size: SwitchSize,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="switch"
            data-size=(size.as_str())
            class=(class!(ROOT, size.root_classes(), attrs.remove("class")))
        >
            <input type="checkbox" role="switch" class=(INPUT) (attrs)>
            <span data-slot="switch-thumb" class=(class!(THUMB, size.thumb_classes()))></span>
        </span>
    })
}
