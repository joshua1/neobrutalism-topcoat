//! A collapsible application sidebar.
//!
//! Octane keeps the sidebar's state in a React-style context. Here the state
//! is a pair of data attributes on the [`sidebar`] element, `data-state`
//! (`expanded` / `collapsed`) and `data-collapsible` (empty, `icon` or
//! `offcanvas`), so every part styles itself with the same
//! `group-data-[collapsible=icon]:` classes as in Octane. A few lines of
//! inline JavaScript flip them:
//!
//! - [`sidebar_trigger`] and [`sidebar_rail`] toggle every sidebar inside
//!   their [`sidebar_provider`], and Ctrl+B / Cmd+B toggles every provider on
//!   the page (a small script the provider renders).
//! - Each toggle writes the `sidebar_state` cookie, like Octane. Read it back
//!   on the server with [`sidebar_state`] and pass it to `default_open` so the
//!   sidebar renders in the state the user left it.
//! - Below the `md` breakpoint the same toggle opens the sidebar as a modal
//!   [`sheet`](super::sheet) instead. The sidebar's children render once: the
//!   sheet's `<dialog>` is `display: contents` from `md` up, so on desktop it
//!   is invisible to layout and the panel sits in the page.
//!
//! Collapsed menu buttons show their `tooltip` with the
//! [`tooltip`](super::tooltip) component's content, placed with CSS anchor
//! positioning so the sidebar's scrolling content does not clip it.
//!
//! Not matched from Octane: there is no `useSidebar` hook or `open` /
//! `onOpenChange` props (read `data-state` from the DOM instead), the state
//! lives in the DOM and is not shared with the Topcoat runtime, and the
//! shortcut script only runs on a full page load.

// Apps often use only some parts of a component (e.g. a select without its
// scroll arrows), so the unused parts' helpers must not warn.
#![allow(dead_code)]

use topcoat::{
    Result,
    context::Cx,
    icon::{icon, iconify::iconify_icon},
    router::request::headers,
    view::{
        Attributes, Child, Class, StaticClass, Unescaped, View, attributes, class, component, view,
    },
};

use super::{
    button::{ButtonSize, ButtonVariant, button_variants},
    input::input,
    sheet::{sheet, sheet_description, sheet_header, sheet_title},
    skeleton::skeleton,
    tooltip::{TooltipAlign, TooltipSide, tooltip_content},
};

/// The cookie the sidebar writes its desktop state to: `true` when expanded,
/// `false` when collapsed. It lasts a week, on path `/`.
#[allow(dead_code)]
pub const SIDEBAR_COOKIE_NAME: &str = "sidebar_state";

/// Reads the `sidebar_state` cookie from the current request: `Some(true)`
/// when the user last left the sidebar expanded, `Some(false)` when
/// collapsed, and `None` when there is no such cookie.
///
/// It parses the `Cookie` header directly, so it works without registering
/// Topcoat's cookie layer. Pass the result to [`sidebar`]'s `default_open`:
///
/// ```ignore
/// #[page]
/// async fn dashboard(cx: &Cx) -> Result<impl View> {
///     let open = sidebar_state(cx).unwrap_or(true);
///     Ok(view! {
///         sidebar_provider(
///             sidebar(default_open: open, collapsible: SidebarCollapsible::Icon, ..)
///             sidebar_inset(..)
///         )
///     })
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn sidebar_state(cx: &Cx) -> Option<bool> {
    headers(cx)
        .get_all("cookie")
        .iter()
        .filter_map(|header| header.to_str().ok())
        .flat_map(|header| header.split(';'))
        .find_map(|pair| match pair.trim().split_once('=')? {
            (SIDEBAR_COOKIE_NAME, "true") => Some(true),
            (SIDEBAR_COOKIE_NAME, "false") => Some(false),
            _ => None,
        })
}

/// The toggle, as a JavaScript function of an element inside a provider (or
/// the provider itself). A macro rather than a `const` so `concat!` can build
/// the inline handlers and the shortcut script from it at compile time.
///
/// On desktop it flips every collapsible sidebar in the provider to the
/// opposite of the first one's state and writes the cookie. Below `md` it
/// opens (or closes) each sidebar's sheet instead, showing it expanded until
/// the sheet closes.
macro_rules! toggle_js {
    () => {
        "(el) => {\
            const w = el.closest('[data-slot=sidebar-wrapper]') ?? el.closest('[data-slot=sidebar]');\
            if (!w) return;\
            const sel = '[data-slot=sidebar][data-collapsible-mode]';\
            const bars = w.matches(sel) ? [w] : [...w.querySelectorAll(sel)];\
            if (!bars.length) return;\
            if (matchMedia('(max-width: 767px)').matches) {\
                for (const s of bars) {\
                    const d = s.querySelector(':scope > [data-sidebar=sheet]');\
                    if (d.open) { d.close(); continue; }\
                    s.dataset.restore = s.dataset.state;\
                    s.dataset.state = 'expanded';\
                    s.dataset.collapsible = '';\
                    d.showModal();\
                }\
                return;\
            }\
            const open = bars[0].dataset.state === 'collapsed';\
            for (const s of bars) {\
                s.dataset.state = open ? 'expanded' : 'collapsed';\
                s.dataset.collapsible = open ? '' : s.dataset.collapsibleMode;\
            }\
            document.cookie = `sidebar_state=${open}; path=/; max-age=604800`;\
        }"
    };
}

/// Inline `onclick` for the trigger and the rail.
const TOGGLE_ONCLICK: &str = concat!("(", toggle_js!(), ")(this)");

/// Toggles every provider on the page on Ctrl+B / Cmd+B. Each provider
/// renders it; the flag keeps the listener from being added twice.
const SHORTCUT_SCRIPT: &str = concat!(
    "if (!window.neoSidebarShortcut) {\
        window.neoSidebarShortcut = true;\
        const toggle = ",
    toggle_js!(),
    ";\
        addEventListener('keydown', (e) => {\
            if (e.key === 'b' && (e.metaKey || e.ctrlKey)) {\
                e.preventDefault();\
                document.querySelectorAll('[data-slot=sidebar-wrapper]').forEach((w) => toggle(w));\
            }\
        });\
    }"
);

/// `onclose` for the mobile sheet: puts back the desktop state that opening
/// the sheet replaced.
const RESTORE_ONCLOSE: &str = "const s = this.parentElement;\
    if (s.dataset.restore) {\
        s.dataset.state = s.dataset.restore;\
        s.dataset.collapsible = s.dataset.restore === 'collapsed' ? s.dataset.collapsibleMode : '';\
        delete s.dataset.restore;\
    }";

/// Classes for the provider. The widths are CSS variables; override them with
/// `style="--sidebar-width: 20rem"` on the provider.
const PROVIDER: StaticClass = class!(
    "group/sidebar-wrapper has-data-[variant=inset]:bg-sidebar flex min-h-svh w-full \
     [--sidebar-width:16rem] [--sidebar-width-icon:3rem]",
);

/// The layout around one or two [`sidebar`]s and a [`sidebar_inset`], and the
/// home of the Ctrl+B / Cmd+B shortcut.
///
/// Octane's `SIDEBAR_WIDTH` and `SIDEBAR_WIDTH_ICON` are the
/// `--sidebar-width` (16rem) and `--sidebar-width-icon` (3rem) CSS variables;
/// set them through `style` in `attrs`. A right sidebar goes after the inset.
///
/// The provider is at least as tall as the viewport. To show a sidebar inside
/// a box instead (a docs preview, a dialog), give the provider a height and
/// `relative` (plus `min-h-0!` to drop the viewport minimum) and set
/// `contained` on the [`sidebar`].
///
/// ```ignore
/// view! {
///     sidebar_provider(
///         sidebar(collapsible: SidebarCollapsible::Icon,
///             sidebar_header(..)
///             sidebar_content(..)
///             sidebar_rail()
///         )
///         sidebar_inset(
///             <header class="flex h-16 items-center gap-2 px-4">
///                 sidebar_trigger()
///             </header>
///         )
///     )
/// }
/// ```
#[component]
pub async fn sidebar_provider(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div data-slot="sidebar-wrapper" class=(class!(PROVIDER, attrs.remove("class"))) (attrs)>
            (child)
            // A trusted constant, so it is written out unescaped.
            <script>(Unescaped::new_unchecked(SHORTCUT_SCRIPT))</script>
        </div>
    })
}

/// The edge of the page a [`sidebar`] sits on.
///
/// [`Default`] is `SidebarSide::Left`, used when no side is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SidebarSide {
    /// The left edge.
    #[default]
    Left,
    /// The right edge. Place the sidebar after the [`sidebar_inset`].
    Right,
}

impl SidebarSide {
    /// The value of the sidebar's `data-side` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Right => "right",
        }
    }

    /// Classes that pin the panel to its edge, and push it past the edge when
    /// it collapses off canvas.
    fn container(self) -> StaticClass {
        match self {
            Self::Left => class!(
                "left-0 group-data-[collapsible=offcanvas]:left-[calc(var(--sidebar-width)*-1)]",
            ),
            Self::Right => class!(
                "right-0 group-data-[collapsible=offcanvas]:right-[calc(var(--sidebar-width)*-1)]",
            ),
        }
    }

    /// Below `md`: classes that slide the sheet in from the edge.
    fn sheet_motion(self) -> StaticClass {
        match self {
            Self::Left => class!(
                "max-md:-translate-x-full max-md:group-open/dialog:translate-x-0 \
                 max-md:starting:group-open/dialog:-translate-x-full",
            ),
            Self::Right => class!(
                "max-md:translate-x-full max-md:group-open/dialog:translate-x-0 \
                 max-md:starting:group-open/dialog:translate-x-full",
            ),
        }
    }
}

/// How a [`sidebar`] looks against the page.
///
/// [`Default`] is `SidebarVariant::Sidebar`, used when no variant is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SidebarVariant {
    /// A full-height panel with a thick border against the page.
    #[default]
    Sidebar,
    /// A panel inset from the edges by padding.
    Floating,
    /// Padding around the panel, with the [`sidebar_inset`] drawn as a card.
    Inset,
}

impl SidebarVariant {
    /// The value of the sidebar's `data-variant` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Sidebar => "sidebar",
            Self::Floating => "floating",
            Self::Inset => "inset",
        }
    }

    /// Whether the panel has padding around it.
    fn is_padded(self) -> bool {
        matches!(self, Self::Floating | Self::Inset)
    }
}

/// What a [`sidebar`] does when it collapses on desktop.
///
/// [`Default`] is `SidebarCollapsible::Offcanvas`, used when none is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SidebarCollapsible {
    /// Slide out past the edge of the page.
    #[default]
    Offcanvas,
    /// Shrink to a strip of icons; labels hide and menu buttons show tooltips.
    Icon,
    /// Never collapse: a plain column, with no sheet on mobile.
    None,
}

impl SidebarCollapsible {
    /// The value `data-collapsible` takes while the sidebar is collapsed.
    fn as_str(self) -> &'static str {
        match self {
            Self::Offcanvas => "offcanvas",
            Self::Icon => "icon",
            Self::None => "none",
        }
    }
}

/// Classes for the panel, as in Octane. Octane's `hidden md:flex` is plain
/// `flex`: below `md` the closed sheet hides it.
const CONTAINER: StaticClass = class!(
    "inset-y-0 z-10 flex w-(--sidebar-width) transition-[left,right,width] duration-200 \
     ease-linear",
);

/// Below `md`: classes that turn the panel into Octane's mobile sheet (18rem
/// wide, bordered, sliding in over 500ms and out over 300ms).
const MOBILE_SHEET: StaticClass = class!(
    "max-md:z-50 max-md:p-0 max-md:[--sidebar-width:18rem] max-md:border-2 max-md:border-border \
     max-md:transition max-md:duration-300 max-md:ease-in-out max-md:group-open/dialog:duration-500",
);

/// The sidebar itself: a panel fixed to one edge of the page (a sheet below
/// `md`), holding a [`sidebar_header`], a [`sidebar_content`], a
/// [`sidebar_footer`] and optionally a [`sidebar_rail`].
///
/// `side` defaults to `Left`, `variant` to `Sidebar` and `collapsible` to
/// `Offcanvas`. `default_open` (default `true`) is the desktop state it
/// renders in; pass [`sidebar_state`] to restore the user's last choice.
///
/// `contained` positions the panel `absolute` and `h-full` instead of
/// `fixed` and `h-svh`, so it fills the nearest positioned ancestor (usually
/// a `relative` [`sidebar_provider`] with a set height) rather than the
/// viewport. Octane's docs preview gets the same effect with `class="static"`.
///
/// `attrs` are forwarded to the panel (Octane's `sidebar-container`), with
/// extra classes appended.
///
/// ```ignore
/// view! {
///     sidebar(
///         collapsible: SidebarCollapsible::Icon,
///         default_open: sidebar_state(cx).unwrap_or(true),
///         sidebar_content(
///             sidebar_group(
///                 sidebar_group_label("Application")
///                 sidebar_menu(
///                     sidebar_menu_item(
///                         sidebar_menu_button(href: "/", tooltip: "Home",
///                             icon(data: iconify_icon!("lucide:house"))
///                             <span>"Home"</span>
///                         )
///                     )
///                 )
///             )
///         )
///     )
/// }
/// ```
#[component]
pub async fn sidebar(
    /// The edge of the page.
    #[default]
    side: SidebarSide,
    /// The panel's look.
    #[default]
    variant: SidebarVariant,
    /// What collapsing does.
    #[default]
    collapsible: SidebarCollapsible,
    /// Whether the desktop sidebar starts expanded.
    #[default(true)]
    default_open: bool,
    /// Fill the nearest positioned ancestor instead of the viewport.
    #[default]
    contained: bool,
    /// Extra attributes for the panel.
    #[default]
    mut attrs: Attributes,
    /// The sidebar's sections.
    #[default]
    child: Child<'_>,
) -> Result<impl View> {
    let extra = attrs.remove("class");
    let state = if default_open {
        "expanded"
    } else {
        "collapsed"
    };
    let collapsed_as = if default_open {
        ""
    } else {
        collapsible.as_str()
    };

    Ok(view! {
        if collapsible == SidebarCollapsible::None {
            <div
                data-slot="sidebar"
                class=(class!(
                    "bg-secondary-background text-foreground flex h-full w-(--sidebar-width) flex-col",
                    extra,
                ))
                (attrs)
            >
                (child)
            </div>
        } else {
            <div
                class="group peer"
                data-state=(state)
                data-collapsible=(collapsed_as)
                data-collapsible-mode=(collapsible.as_str())
                data-variant=(variant.as_str())
                data-side=(side.as_str())
                data-slot="sidebar"
            >
                // Reserves the panel's width in the page on desktop.
                <div
                    data-slot="sidebar-gap"
                    class=(class!(
                        "hidden md:block",
                        "relative w-(--sidebar-width) bg-transparent transition-[width] duration-200 ease-linear",
                        "group-data-[collapsible=offcanvas]:w-0",
                        "group-data-[side=right]:rotate-180",
                        "group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4)))]"
                            if variant.is_padded()
                            else "group-data-[collapsible=icon]:w-(--sidebar-width-icon)",
                    ))
                ></div>
                sheet(
                    attrs: attributes! {
                        data-sidebar="sheet"
                        aria-label="Sidebar"
                        onclose=(RESTORE_ONCLOSE)
                        class="md:contents"
                    },
                    <div
                        data-slot="sidebar-container"
                        class=(class!(
                            "absolute h-full" if contained else "fixed h-svh",
                            CONTAINER,
                            side.container(),
                            "p-2 group-data-[collapsible=icon]:w-[calc(var(--sidebar-width-icon)+(--spacing(4))+2px)]"
                                if variant.is_padded()
                                else "group-data-[collapsible=icon]:w-(--sidebar-width-icon) \
                                      group-data-[side=left]:border-r-2 border-r-border \
                                      group-data-[side=right]:border-l-2 border-l-border",
                            MOBILE_SHEET,
                            side.sheet_motion(),
                            extra,
                        ))
                        (attrs)
                    >
                        sheet_header(
                            attrs: attributes! { class="sr-only md:hidden" },
                            sheet_title("Sidebar")
                            sheet_description("Displays the mobile sidebar.")
                        )
                        <div
                            data-sidebar="sidebar"
                            data-slot="sidebar-inner"
                            class="bg-secondary-background flex h-full w-full flex-col"
                        >
                            (child)
                        </div>
                    </div>
                )
            </div>
        }
    })
}

/// A small button that toggles the sidebars in its [`sidebar_provider`]
/// (opens the sheet below `md`), showing a panel icon.
///
/// `attrs` are forwarded to the `<button>`, with extra classes appended. It
/// sets `onclick` itself; use `@click` to run more code on click.
#[component]
pub async fn sidebar_trigger(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-sidebar="trigger"
            data-slot="sidebar-trigger"
            onclick=(TOGGLE_ONCLICK)
            // `!` replaces the icon button's `size-10`.
            class=(class!(
                button_variants(ButtonVariant::NoShadow, ButtonSize::Icon),
                "size-7!",
                attrs.remove("class"),
            ))
            (attrs)
        >
            icon(data: iconify_icon!("lucide:panel-left"))
            <span class="sr-only">"Toggle Sidebar"</span>
        </button>
    })
}

/// An invisible strip along the sidebar's inner edge that toggles it on
/// click, with a resize cursor. Place it last inside the [`sidebar`].
#[component]
pub async fn sidebar_rail(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-sidebar="rail"
            data-slot="sidebar-rail"
            aria-label="Toggle Sidebar"
            tabindex="-1"
            title="Toggle Sidebar"
            onclick=(TOGGLE_ONCLICK)
            class=(class!(
                "absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear \
                 group-data-[side=left]:-right-4 group-data-[side=right]:left-0 after:absolute \
                 after:inset-y-0 after:left-1/2 after:w-[2px] sm:flex",
                "in-data-[side=left]:cursor-w-resize in-data-[side=right]:cursor-e-resize",
                "[[data-side=left][data-state=collapsed]_&]:cursor-e-resize \
                 [[data-side=right][data-state=collapsed]_&]:cursor-w-resize",
                "hover:group-data-[collapsible=offcanvas]:bg-sidebar \
                 group-data-[collapsible=offcanvas]:translate-x-0 \
                 group-data-[collapsible=offcanvas]:after:left-full",
                "[[data-side=left][data-collapsible=offcanvas]_&]:-right-2",
                "[[data-side=right][data-collapsible=offcanvas]_&]:-left-2",
                attrs.remove("class"),
            ))
            (attrs)
        ></button>
    })
}

/// The page's main content beside the sidebar, rendered as a `<main>`. With
/// the `Inset` variant it is drawn as a rounded card.
#[component]
pub async fn sidebar_inset(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <main
            data-slot="sidebar-inset"
            class=(class!(
                "bg-secondary-background relative flex w-full flex-1 flex-col",
                "md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 \
                 md:peer-data-[variant=inset]:rounded-base md:peer-data-[variant=inset]:shadow-sm \
                 md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </main>
    })
}

/// A compact [`input`] for the sidebar, such as a search field.
///
/// Every attribute in `attrs` goes on the `<input>`.
#[component]
pub async fn sidebar_input(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        input(attrs: attributes! {
            data-slot="sidebar-input"
            data-sidebar="input"
            // `!` replaces the input's own `h-10`.
            class=(class!("bg-secondary-background h-8! w-full shadow-none", attrs.remove("class")))
            (attrs)
        })
    })
}

/// The top section of a [`sidebar`], above the scrolling content, with a
/// thick bottom border.
#[component]
pub async fn sidebar_header(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-header"
            data-sidebar="header"
            class=(class!("flex flex-col gap-2 p-2 border-b-2 border-b-border", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The bottom section of a [`sidebar`], below the scrolling content, with a
/// thick top border.
#[component]
pub async fn sidebar_footer(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-footer"
            data-sidebar="footer"
            class=(class!("flex flex-col gap-2 p-2 border-t-2 border-t-border", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A thick rule between sections of a [`sidebar`].
///
/// Octane has no separator (groups draw their own borders); this one is
/// provided for markup ported from shadcn/ui.
#[component]
#[allow(dead_code)]
pub async fn sidebar_separator(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-separator"
            data-sidebar="separator"
            role="separator"
            class=(class!("mx-2 h-0.5 w-auto shrink-0 bg-border", attrs.remove("class")))
            (attrs)
        ></div>
    })
}

/// The scrolling middle of a [`sidebar`], holding its [`sidebar_group`]s.
#[component]
pub async fn sidebar_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-content"
            data-sidebar="content"
            class=(class!(
                "flex min-h-0 flex-1 flex-col overflow-auto group-data-[collapsible=icon]:overflow-hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A section of a [`sidebar_content`], with a thick border below all but the
/// last.
#[component]
pub async fn sidebar_group(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-group"
            data-sidebar="group"
            class=(class!(
                "relative flex w-full min-w-0 flex-col p-2 border-b-2 border-b-border last:border-b-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// The heading of a [`sidebar_group`]. It fades out when the sidebar
/// collapses to icons.
#[component]
pub async fn sidebar_group_label(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-group-label"
            data-sidebar="group-label"
            class=(class!(
                "text-foreground ring-ring flex h-8 shrink-0 items-center rounded-base px-2 text-sm \
                 font-heading outline-hidden transition-[margin,opacity] duration-200 ease-linear \
                 focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0",
                "group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// An icon button in the top-right corner of a [`sidebar_group`], such as
/// "add project". Give it an `aria-label` or a `sr-only` label.
#[component]
#[allow(dead_code)]
pub async fn sidebar_group_action(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="sidebar-group-action"
            data-sidebar="group-action"
            class=(class!(
                "absolute top-3.5 right-3 flex aspect-square w-5 items-center justify-center \
                 rounded-base p-0 outline-hidden transition-transform focus-visible:ring-2 \
                 [&>svg]:size-4 [&>svg]:shrink-0",
                // Increases the hit area of the button on mobile.
                "after:absolute after:-inset-2 md:after:hidden",
                "group-data-[collapsible=icon]:hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </button>
    })
}

/// The body of a [`sidebar_group`], usually a [`sidebar_menu`].
#[component]
pub async fn sidebar_group_content(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-group-content"
            data-sidebar="group-content"
            class=(class!("w-full text-sm", attrs.remove("class")))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A list of [`sidebar_menu_item`]s, rendered as a `<ul>`.
#[component]
pub async fn sidebar_menu(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            data-slot="sidebar-menu"
            data-sidebar="menu"
            class=(class!("flex w-full min-w-0 flex-col gap-1", attrs.remove("class")))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One entry of a [`sidebar_menu`], rendered as an `<li>`: a
/// [`sidebar_menu_button`] and optionally an action, a badge or a submenu.
///
/// It is also the hover group and the anchor of a collapsed button's
/// [`sidebar_menu_tooltip`] (`group/tooltip`, `anchor-name`).
#[component]
pub async fn sidebar_menu_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-slot="sidebar-menu-item"
            data-sidebar="menu-item"
            class=(class!(
                "group/menu-item relative font-base",
                "group/tooltip [anchor-name:--sidebar-menu-item] [anchor-scope:--sidebar-menu-item]",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </li>
    })
}

/// The height and text size of a [`sidebar_menu_button`].
///
/// [`Default`] is `SidebarMenuButtonSize::Default`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SidebarMenuButtonSize {
    /// A 32px row.
    #[default]
    Default,
    /// A 28px row with small text.
    Sm,
    /// A 48px row, for an avatar or logo with two lines of text.
    Lg,
}

impl SidebarMenuButtonSize {
    /// The value of the button's `data-size` attribute.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Sm => "sm",
            Self::Lg => "lg",
        }
    }

    /// Classes for the size.
    fn classes(self) -> StaticClass {
        match self {
            Self::Default => class!("h-8 text-sm"),
            Self::Sm => class!("h-7 text-xs"),
            Self::Lg => class!("h-12 text-sm group-data-[collapsible=icon]:p-0!"),
        }
    }
}

/// Classes shared by every menu button size.
const MENU_BUTTON: StaticClass = class!(
    "peer/menu-button flex w-full items-center gap-2 overflow-hidden outline-2 outline-transparent \
     rounded-base p-2 text-left text-sm ring-ring transition-[width,height,padding] hover:bg-main \
     hover:text-main-foreground hover:outline-border focus-visible:outline-border \
     focus-visible:text-main-foreground focus-visible:bg-main disabled:pointer-events-none \
     disabled:opacity-50 group-has-data-[sidebar=menu-action]/menu-item:pr-8 \
     aria-disabled:pointer-events-none aria-disabled:opacity-50 data-active:bg-main \
     data-active:text-main-foreground data-active:outline-border \
     group-data-[collapsible=icon]:size-8! group-data-[collapsible=icon]:p-2! \
     [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
);

/// Builds the classes of a [`sidebar_menu_button`] of the given `size`.
///
/// Octane renders other elements as menu buttons with `render={..}`; here you
/// give them these classes. Add `data-sidebar="menu-button"` and
/// `data-size=(size.as_str())` too, which the badge and action read:
///
/// ```ignore
/// view! {
///     collapsible_trigger(
///         attrs: attributes! {
///             data-sidebar="menu-button"
///             data-size="default"
///             class=(sidebar_menu_button_variants(SidebarMenuButtonSize::Default))
///         },
///         <span>"Models"</span>
///     )
/// }
/// ```
#[must_use]
#[allow(dead_code)]
pub fn sidebar_menu_button_variants(
    size: SidebarMenuButtonSize,
) -> Class<(StaticClass, StaticClass)> {
    class!(MENU_BUTTON, size.classes())
}

/// The main button of a [`sidebar_menu_item`]: an icon followed by a
/// `<span>` label. The label truncates, and hides when the sidebar collapses
/// to icons.
///
/// With `href` it renders an `<a>` (Octane's `render={<a href=.. />}`),
/// otherwise a `<button>`. `active` highlights it. `tooltip` is shown beside
/// it while the sidebar is collapsed to icons. `attrs` are forwarded to the
/// element, with extra classes appended.
///
/// ```ignore
/// view! {
///     sidebar_menu_item(
///         sidebar_menu_button(href: "/inbox", active: true, tooltip: "Inbox",
///             icon(data: iconify_icon!("lucide:inbox"))
///             <span>"Inbox"</span>
///         )
///         sidebar_menu_badge("24")
///     )
/// }
/// ```
#[component]
pub async fn sidebar_menu_button(
    /// The row height.
    #[default]
    size: SidebarMenuButtonSize,
    /// Highlights the button as the current page.
    #[default]
    active: bool,
    /// Text shown beside the button while the sidebar is collapsed to icons.
    #[into]
    #[default]
    tooltip: Option<&str>,
    /// Renders an `<a>` to this URL instead of a `<button>`.
    #[into]
    #[default]
    href: Option<&str>,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let attrs = attributes! {
        data-slot="sidebar-menu-button"
        data-sidebar="menu-button"
        data-size=(size.as_str())
        data-active=(active)
        class=(class!(sidebar_menu_button_variants(size), attrs.remove("class")))
        (attrs)
    };

    Ok(view! {
        if let Some(href) = href {
            <a href=(href) (attrs)>(child)</a>
        } else {
            <button type="button" (attrs)>(child)</button>
        }
        if let Some(tooltip) = tooltip {
            sidebar_menu_tooltip((tooltip))
        }
    })
}

/// Classes that show the hint only while the sidebar is collapsed to icons,
/// and move it out of the sidebar's clipping box: `fixed` with CSS anchor
/// positioning, to the right of its [`sidebar_menu_item`]. `!` overrides the
/// tooltip's own `absolute` placement.
const MENU_TOOLTIP: StaticClass = class!(
    "hidden group-data-[collapsible=icon]:block fixed! inset-auto! \
     [position-anchor:--sidebar-menu-item] [position-area:right]",
);

/// The hint a collapsed [`sidebar_menu_button`] shows on hover and focus, a
/// [`tooltip_content`] beside its [`sidebar_menu_item`].
///
/// `sidebar_menu_button(tooltip: ..)` renders one. Use it directly when the
/// button is another element, such as a `collapsible_trigger`: place it in
/// the same [`sidebar_menu_item`], outside any `<details>`.
#[component]
pub async fn sidebar_menu_tooltip(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        tooltip_content(
            side: TooltipSide::Right,
            align: TooltipAlign::Start,
            attrs: attributes! { class=(class!(MENU_TOOLTIP, attrs.remove("class"))) (attrs) },
            (child)
        )
    })
}

/// Classes for a menu action.
const MENU_ACTION: StaticClass = class!(
    "[&_svg]:text-foreground hover:[&_svg]:text-main-foreground text-main-foreground hover:bg-main \
     hover:outline-border outline-transparent outline-2 absolute top-1.5 right-1 flex aspect-square \
     w-5 items-center justify-center rounded-base p-0 transition-transform focus-visible:ring-2 \
     [&>svg]:size-4 [&>svg]:shrink-0 \
     after:absolute after:-inset-2 md:after:hidden \
     peer-data-[size=sm]/menu-button:top-1 peer-data-[size=default]/menu-button:top-1.5 \
     peer-data-[size=lg]/menu-button:top-2.5 group-data-[collapsible=icon]:hidden",
);

/// Classes that hide a menu action until its item is hovered or focused.
const SHOW_ON_HOVER: StaticClass = class!(
    "group-focus-within/menu-item:opacity-100 group-hover/menu-item:opacity-100 \
     peer-data-active/menu-button:text-main-foreground aria-expanded:opacity-100 md:opacity-0",
);

/// Builds the classes of a [`sidebar_menu_action`], for another element
/// acting as one (such as a `dropdown_menu_trigger`). Add
/// `data-sidebar="menu-action"` too, which makes room for it in the button.
#[must_use]
#[allow(dead_code)]
pub fn sidebar_menu_action_variants(
    show_on_hover: bool,
) -> Class<(StaticClass, Option<StaticClass>)> {
    class!(MENU_ACTION, show_on_hover.then_some(SHOW_ON_HOVER))
}

/// A small icon button at the end of a [`sidebar_menu_item`]. With
/// `show_on_hover` it appears only while the item is hovered or focused (on
/// desktop). Give it an `aria-label` or a `sr-only` label.
#[component]
#[allow(dead_code)]
pub async fn sidebar_menu_action(
    /// Hides the action until its item is hovered or focused.
    #[default]
    show_on_hover: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <button
            type="button"
            data-slot="sidebar-menu-action"
            data-sidebar="menu-action"
            class=(class!(sidebar_menu_action_variants(show_on_hover), attrs.remove("class")))
            (attrs)
        >
            (child)
        </button>
    })
}

/// A count or status at the end of a [`sidebar_menu_item`]. Place it after
/// the button.
#[component]
#[allow(dead_code)]
pub async fn sidebar_menu_badge(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div
            data-slot="sidebar-menu-badge"
            data-sidebar="menu-badge"
            class=(class!(
                "text-foreground pointer-events-none absolute right-1 flex h-5 min-w-5 items-center \
                 justify-center rounded-base px-1 text-xs font-base tabular-nums select-none",
                "peer-hover/menu-button:text-main-foreground",
                "peer-data-[size=sm]/menu-button:top-1",
                "peer-data-[size=default]/menu-button:top-1.5",
                "peer-data-[size=lg]/menu-button:top-2.5",
                "group-data-[collapsible=icon]:hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </div>
    })
}

/// A random whole number from 50 to 89, for a skeleton's width.
///
/// The standard library has no random number generator, but every new
/// `RandomState` is seeded randomly, so hashing nothing with it gives a fresh
/// number each call.
fn random_percent() -> u64 {
    use std::hash::{BuildHasher, Hasher};

    let random = std::collections::hash_map::RandomState::new()
        .build_hasher()
        .finish();
    random % 40 + 50
}

/// A placeholder row for a [`sidebar_menu`] that is still loading, with an
/// optional icon and a text bar of random width (50 to 90%).
#[component]
#[allow(dead_code)]
pub async fn sidebar_menu_skeleton(
    /// Shows an icon placeholder before the text.
    #[default]
    show_icon: bool,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    let width = format!("--skeleton-width: {}%", random_percent());

    Ok(view! {
        <div
            data-slot="sidebar-menu-skeleton"
            data-sidebar="menu-skeleton"
            class=(class!("flex h-8 items-center gap-2 rounded-base px-2", attrs.remove("class")))
            (attrs)
        >
            if show_icon {
                skeleton(attrs: attributes! {
                    class="size-4 rounded-base"
                    data-sidebar="menu-skeleton-icon"
                })
            }
            skeleton(attrs: attributes! {
                class="h-4 max-w-(--skeleton-width) flex-1"
                data-sidebar="menu-skeleton-text"
                style=(width)
            })
        </div>
    })
}

/// A nested list under a [`sidebar_menu_item`], indented with a rule on its
/// left. It hides when the sidebar collapses to icons.
#[component]
pub async fn sidebar_menu_sub(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <ul
            data-slot="sidebar-menu-sub"
            data-sidebar="menu-sub"
            class=(class!(
                "border-l-foreground/50 mx-3.5 flex min-w-0 translate-x-px flex-col gap-1 border-l-2 \
                 px-2.5 py-0.5",
                "group-data-[collapsible=icon]:hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </ul>
    })
}

/// One entry of a [`sidebar_menu_sub`], rendered as an `<li>`.
#[component]
pub async fn sidebar_menu_sub_item(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <li
            data-slot="sidebar-menu-sub-item"
            data-sidebar="menu-sub-item"
            class=(class!("group/menu-sub-item relative", attrs.remove("class")))
            (attrs)
        >
            (child)
        </li>
    })
}

/// The text size of a [`sidebar_menu_sub_button`].
///
/// [`Default`] is `SidebarMenuSubButtonSize::Md`, used when no size is given.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SidebarMenuSubButtonSize {
    /// Small text.
    Sm,
    /// The standard text size.
    #[default]
    Md,
}

impl SidebarMenuSubButtonSize {
    /// The value of the link's `data-size` attribute.
    fn as_str(self) -> &'static str {
        match self {
            Self::Sm => "sm",
            Self::Md => "md",
        }
    }

    /// Classes for the size.
    fn classes(self) -> StaticClass {
        match self {
            Self::Sm => class!("text-xs"),
            Self::Md => class!("text-sm"),
        }
    }
}

/// A link in a [`sidebar_menu_sub_item`], rendered as an `<a>`. Pass `href`
/// in `attrs`; `active` highlights it.
#[component]
pub async fn sidebar_menu_sub_button(
    /// The text size.
    #[default]
    size: SidebarMenuSubButtonSize,
    /// Highlights the link as the current page.
    #[default]
    active: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <a
            data-slot="sidebar-menu-sub-button"
            data-sidebar="menu-sub-button"
            data-size=(size.as_str())
            data-active=(active)
            class=(class!(
                "text-foreground hover:bg-main hover:outline-border hover:text-main-foreground \
                 active:bg-main outline-transparent outline-2 [&>svg]:text-main-foreground flex h-7 \
                 min-w-0 -translate-x-px items-center gap-2 overflow-hidden rounded-base px-2 \
                 focus-visible:ring-2 disabled:pointer-events-none disabled:opacity-50 \
                 aria-disabled:pointer-events-none aria-disabled:opacity-50 \
                 [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0",
                "data-active:bg-main data-active:outline-border",
                size.classes(),
                "group-data-[collapsible=icon]:hidden",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </a>
    })
}
