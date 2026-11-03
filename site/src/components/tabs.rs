// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    runtime::Expr,
    view::{Attributes, Child, Class, StaticClass, View, class, component, view},
};

/// How a [`tabs`] lays out its list and panels.
///
/// [`Default`] is `TabsOrientation::Horizontal`, used when no orientation is
/// given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TabsOrientation {
    /// The triggers sit in a row above the panels.
    #[default]
    Horizontal,
    /// The triggers stack in a column beside the panels.
    Vertical,
}

/// Selects the trigger that was clicked and shows the panel with the same
/// `data-value`, in the browser. Parts of tabs nested inside this one are left
/// alone.
const SELECT_JS: &str = "const r=this.closest('[data-slot=tabs]'),v=this.dataset.value;\
     r.querySelectorAll('[data-slot=tabs-trigger],[data-slot=tabs-content]').forEach(e=>{\
     if(e.closest('[data-slot=tabs]')!==r)return;const on=e.dataset.value===v;\
     if(e.dataset.slot==='tabs-content'){e.hidden=!on;return}\
     e.toggleAttribute('data-active',on);e.setAttribute('aria-selected',on)})";

/// A set of panels, one visible at a time, with a list of triggers to pick it.
///
/// Triggers work in one of two ways:
///
/// - **In the browser (default).** A trigger is a `<button>`; clicking it
///   selects it and shows the [`tabs_content`] with the same `value`, through
///   a small inline handler and no server round trip. Mark the initially
///   selected trigger and panel with `active: true`.
/// - **As links.** Give a trigger an `href` in `attrs` and it renders as an
///   `<a>` with `aria-current="page"` while `active`, so each tab is its own
///   URL. Render only the selected panel on the server.
///
/// `active` also accepts a runtime expression, so a selected-tab signal can
/// drive the triggers and panels instead.
///
/// The ARIA tabs pattern's arrow-key navigation is not implemented; triggers
/// use normal Tab navigation. `attrs` are forwarded to the outer `<div>`, with
/// extra classes appended to its classes.
///
/// ```ignore
/// view! {
///     tabs(
///         tabs_list(
///             tabs_trigger(value: "account", active: true, "Account")
///             tabs_trigger(value: "password", "Password")
///         )
///         tabs_content(value: "account", active: true, "Account settings")
///         tabs_content(value: "password", "Password settings")
///     )
/// }
/// ```
#[component]
pub async fn tabs(
    /// Whether the triggers sit above or beside the panels.
    #[default]
    orientation: TabsOrientation,
    /// Extra attributes for the outer `<div>`.
    #[default]
    mut attrs: Attributes,
    /// A [`tabs_list`] followed by the [`tabs_content`] panels.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="tabs"
            match orientation {
                TabsOrientation::Horizontal => data-orientation="horizontal",
                TabsOrientation::Vertical => data-orientation="vertical",
            }
            class=(class!(
                "group/tabs w-full data-[orientation=vertical]:flex \
                 data-[orientation=vertical]:items-start data-[orientation=vertical]:gap-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The visual style of a [`tabs_list`].
///
/// [`Default`] is `TabsListVariant::Default`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TabsListVariant {
    /// A bordered box around the triggers; the selected one is filled.
    #[default]
    Default,
    /// A rule under the triggers; the selected one gets a thick underline.
    Line,
}

impl TabsListVariant {
    /// Classes for the list's frame.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("h-12 rounded-base border-2 border-border bg-background p-1"),
            Self::Line => class!(
                "gap-1 border-b-2 border-border bg-transparent \
                 group-data-[orientation=vertical]/tabs:border-r-2 \
                 group-data-[orientation=vertical]/tabs:border-b-0",
            ),
        }
    }
}

/// Classes shared by both list variants. The orientation comes from the
/// enclosing [`tabs`].
const LIST: StaticClass = class!(
    "group/tabs-list inline-flex items-center justify-center text-foreground \
     group-data-[orientation=vertical]/tabs:h-fit group-data-[orientation=vertical]/tabs:flex-col \
     group-data-[orientation=vertical]/tabs:items-stretch",
);

/// Builds the full class list for a tabs list of the given `variant`.
#[must_use]
#[allow(dead_code)]
pub fn tabs_list_variants(variant: TabsListVariant) -> Class<(StaticClass, StaticClass)> {
    class!(LIST, variant.classes())
}

/// The row (or column) of [`tabs_trigger`]s.
#[component]
pub async fn tabs_list(
    /// A boxed or underlined look.
    #[default]
    variant: TabsListVariant,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="tabs-list"
            role="tablist"
            match variant {
                TabsListVariant::Default => data-variant="default",
                TabsListVariant::Line => data-variant="line",
            }
            class=(class!(LIST, variant.classes(), attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// Classes for a trigger: the selected fill, the line variant's underline,
/// and the vertical layout.
const TRIGGER: StaticClass = class!(
    "relative inline-flex items-center justify-center whitespace-nowrap rounded-base border-2 \
     border-transparent px-2 py-1 gap-1.5 text-sm font-heading ring-offset-white transition-all \
     focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring \
     focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 \
     data-active:bg-main data-active:text-main-foreground data-active:border-border \
     group-data-[orientation=vertical]/tabs:w-full \
     group-data-[orientation=vertical]/tabs:justify-start \
     [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
    "after:pointer-events-none after:absolute after:bg-main after:opacity-0 \
     after:transition-opacity after:hidden group-data-[variant=line]/tabs-list:after:block \
     group-data-[orientation=horizontal]/tabs:after:inset-x-0 \
     group-data-[orientation=horizontal]/tabs:after:-bottom-0.5 \
     group-data-[orientation=horizontal]/tabs:after:h-1 \
     group-data-[orientation=vertical]/tabs:after:inset-y-0 \
     group-data-[orientation=vertical]/tabs:after:-right-0.5 \
     group-data-[orientation=vertical]/tabs:after:w-1",
    "group-data-[variant=line]/tabs-list:data-active:border-transparent \
     group-data-[variant=line]/tabs-list:data-active:bg-transparent \
     group-data-[variant=line]/tabs-list:data-active:text-foreground \
     group-data-[variant=line]/tabs-list:data-active:after:opacity-100",
);

/// A control that selects a panel.
///
/// `value` names the [`tabs_content`] it shows. `active` marks the selected
/// trigger (a boolean or a runtime expression). Without an `href` in `attrs`
/// it is a `<button>` that switches panels in the browser; with one it is an
/// `<a>` that navigates, getting `aria-current="page"` while active. Add
/// `disabled=""` through `attrs` to disable a button trigger.
#[component]
pub async fn tabs_trigger(
    /// The `value` of the panel this trigger shows.
    #[default]
    value: &str,
    /// Whether this trigger is the selected one.
    #[into]
    #[default(false.into())]
    active: Expr<bool>,
    /// Extra attributes for the `<button>` or `<a>`.
    #[default]
    mut attrs: Attributes,
    /// The trigger's label, optionally with an icon.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let is_link = attrs.contains_key("href");
    let class = class!(TRIGGER, attrs.remove("class"));

    Ok(view! {
        if is_link {
            <a
                data-slot="tabs-trigger"
                data-value=(value)
                :data-active=$(active)
                :aria-current=$(active.then_some("page"))
                class=(class)
                (attrs)
            >
                (child)
            </a>
        } else {
            <button
                type="button"
                role="tab"
                data-slot="tabs-trigger"
                data-value=(value)
                :data-active=$(active)
                :aria-selected=$(if active { "true" } else { "false" })
                onclick=(SELECT_JS)
                class=(class)
                (attrs)
            >
                (child)
            </button>
        }
    })
}

/// A panel shown when the [`tabs_trigger`] with the same `value` is selected.
///
/// Set `active` on the initially selected panel; the others render `hidden`.
#[component]
pub async fn tabs_content(
    /// The value its trigger selects.
    #[default]
    value: &str,
    /// Whether this panel is the visible one.
    #[into]
    #[default(false.into())]
    active: Expr<bool>,
    /// Extra attributes for the panel's `<div>`.
    #[default]
    mut attrs: Attributes,
    /// The panel's content.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="tabs-content"
            role="tabpanel"
            tabindex="0"
            data-value=(value)
            :hidden=$(!active)
            class=(class!(
                "mt-2 ring-offset-white focus-visible:outline-none focus-visible:ring-2 \
                 focus-visible:ring-ring focus-visible:ring-offset-2 \
                 group-data-[orientation=vertical]/tabs:mt-0 \
                 group-data-[orientation=vertical]/tabs:flex-1 [&[hidden]]:hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}
