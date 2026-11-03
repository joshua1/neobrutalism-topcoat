//! `/styling`: the theme generator.
//!
//! The Customize sheet is one `<form>`. Its inline `oninput` script applies
//! each change to the page at once and saves the whole form in the
//! `neo-theme` cookie. Everything else happens in Rust: the server reads the
//! cookie into a [`Theme`] (validating it), renders every page with it, and
//! generates the CSS the Copy dialog shows from `/styling/theme.css`.

use topcoat::{
    Result,
    context::Cx,
    router::{page, route},
    view::{Attributes, Child, View, attributes, component, view},
};

use crate::{
    components::{
        button::{ButtonVariant, button, button_variants},
        dialog::{
            dialog, dialog_content, dialog_description, dialog_header, dialog_title, dialog_trigger,
        },
        native_select::{native_select, native_select_option},
        sheet::{
            sheet, sheet_close, sheet_content, sheet_footer, sheet_header, sheet_title,
            sheet_trigger,
        },
    },
    styling_demos::styling_examples,
    theme::{
        BASE_WEIGHTS, COLOR_SLOTS, FONTS, HEADING_WEIGHTS, Mode, PRESETS, RADII, SHADOW_OFFSETS,
        Theme,
    },
};

/// The visitor's theme, or the default one.
fn theme(cx: &Cx) -> Theme {
    super::visitor_theme(cx).unwrap_or_default()
}

/// The full `styles.css` for a Topcoat app, generated from the cookie.
#[route(GET "/styling/theme.css")]
async fn theme_css(cx: &Cx) -> Result<String> {
    Ok(theme(cx).stylesheet())
}

/// Only the `:root` block, for apps that already have the theme installed.
#[route(GET "/styling/root.css")]
async fn root_css(cx: &Cx) -> Result<String> {
    Ok(theme(cx).root_block())
}

#[page]
async fn styling(cx: &Cx) -> Result<impl View> {
    let theme = theme(cx);
    let stylesheet = theme.stylesheet();
    let root_block = theme.root_block();

    Ok(view! {
        <main class="min-h-dvh bg-background bg-[linear-gradient(to_right,#80808033_1px,transparent_1px),linear-gradient(to_bottom,#80808033_1px,transparent_1px)] bg-size-[70px_70px] px-5 pt-[70px]">
            <div class="mx-auto w-[1300px] max-w-full py-16 text-left text-foreground">
                <div class="mb-[50px] flex flex-col gap-8">
                    <h1 class="text-center font-heading text-2xl xl:text-3xl 2xl:text-4xl">"Styling"</h1>
                    <p class="text-center font-base text-base sm:text-lg xl:text-xl 2xl:text-2xl">
                        "Pick any colors, radius, shadow and font, then copy the theme into your Topcoat app."
                    </p>
                    <div class="flex items-center justify-center gap-4">
                        sheet_trigger(target: "customize", attrs: attributes! { class=(button_variants(ButtonVariant::Default, Default::default())) }, "Customize")
                        dialog_trigger(target: "copy-theme", attrs: attributes! { class=(button_variants(ButtonVariant::Neutral, Default::default())) }, "Copy")
                    </div>
                    customize_sheet(theme: &theme)
                    copy_dialog(stylesheet: &stylesheet, root_block: &root_block)
                    styling_examples()
                </div>
            </div>
        </main>
    })
}

/// Applies a form change to the page and saves the form in the cookie.
///
/// `this` is the form. A preset copies its colors into the color inputs
/// first. The cookie uses the `key=value&...` format `Theme::from_cookie`
/// reads; the server validates it, so this script only has to be quick.
const THEME_SCRIPT: &str = r#"
const form = this, root = document.documentElement.style, field = (n) => form.elements[n];
const target = event.target;
if (target.name === 'preset') {
  for (const [name, value] of Object.entries(JSON.parse(target.dataset.colors))) field(name).value = value;
}
for (const input of form.querySelectorAll('input[type=color]')) {
  root.setProperty(input.dataset.variable, input.value);
  input.closest('label').querySelector('[data-hex]').textContent = input.value;
}
root.setProperty('--ring', field('border').value);
root.setProperty('--chart-active-dot', field('border').value);
root.setProperty('--border-radius', field('r').value + 'px');
root.setProperty('--box-shadow-x', field('x').value + 'px');
root.setProperty('--box-shadow-y', field('y').value + 'px');
root.setProperty('--reverse-box-shadow-x', -field('x').value + 'px');
root.setProperty('--reverse-box-shadow-y', -field('y').value + 'px');
root.setProperty('--heading-font-weight', field('hw').value);
root.setProperty('--base-font-weight', field('bw').value);
const font = field('font').selectedOptions[0];
document.getElementById('neo-font').href = font.dataset.href;
root.setProperty('--base-font-family', '"' + font.value + '", sans-serif');
const pairs = [];
for (const [name, value] of new FormData(form)) {
  if (name !== 'preset') pairs.push(name + '=' + String(value).replace('#', '').replaceAll(' ', '+'));
}
document.cookie = 'neo-theme=' + pairs.join('&') + '; path=/; max-age=31536000; samesite=lax';
clearTimeout(form.refresh);
form.refresh = setTimeout(async () => {
  document.getElementById('theme-css').textContent = await (await fetch('/styling/theme.css')).text();
  document.getElementById('root-css').textContent = await (await fetch('/styling/root.css')).text();
}, 200);
"#;

/// Forgets the saved theme and reloads with the default one.
const RESET_SCRIPT: &str = "document.cookie = 'neo-theme=; path=/; max-age=0'; location.reload()";

/// A radio styled as one of the sheet's small option buttons.
const OPTION: &str = "flex h-8 cursor-pointer items-center justify-center rounded-base border-2 \
     border-border bg-secondary-background text-sm font-base text-foreground \
     has-checked:bg-main has-checked:text-main-foreground has-focus-visible:ring-2 \
     has-focus-visible:ring-ring";

/// The Customize sheet: every setting in one form.
#[component]
async fn customize_sheet(theme: &Theme) -> Result<impl View> {
    let radii: Vec<(String, String, bool)> = RADII
        .iter()
        .map(|r| (r.to_string(), format!("{r} px"), *r == theme.radius))
        .collect();
    let shadow_x: Vec<(String, String, bool)> = SHADOW_OFFSETS
        .iter()
        .map(|x| (x.to_string(), format!("{x} px"), *x == theme.shadow_x))
        .collect();
    let shadow_y: Vec<(String, String, bool)> = SHADOW_OFFSETS
        .iter()
        .map(|y| (y.to_string(), format!("{y} px"), *y == theme.shadow_y))
        .collect();
    let heading: Vec<(String, String, bool)> = HEADING_WEIGHTS
        .iter()
        .map(|w| (w.to_string(), w.to_string(), *w == theme.heading_weight))
        .collect();
    let base: Vec<(String, String, bool)> = BASE_WEIGHTS
        .iter()
        .map(|w| (w.to_string(), w.to_string(), *w == theme.base_weight))
        .collect();

    Ok(view! {
        sheet(
            id: "customize",
            // Keep the page visible behind the panel so changes show live.
            attrs: attributes! { class="bg-transparent!" },
            sheet_content(
                sheet_header(sheet_title("Customize styling"))
                <form
                    id="theme-form"
                    class="group/theme grid flex-1 auto-rows-min gap-6 overflow-y-auto px-4 pb-2"
                    oninput=(THEME_SCRIPT)
                    onsubmit="event.preventDefault()"
                >
                    setting(title: "Color mode",
                        <div class="grid grid-cols-2 gap-2">
                            for mode in Mode::ALL {
                                <label class=(OPTION)>
                                    <input class="sr-only" type="radio" name="mode" value=(mode.as_str()) checked=(mode == theme.mode)>
                                    (mode.label())
                                </label>
                            }
                        </div>
                    )
                    setting(title: "Start from a palette",
                        presets()
                    )
                    setting(title: "Colors",
                        <div class="grid gap-2">
                            for slot in COLOR_SLOTS {
                                <label class="flex cursor-pointer items-center gap-3 rounded-base border-2 border-border bg-secondary-background px-2 py-1.5 text-sm font-base text-foreground has-focus-visible:ring-2 has-focus-visible:ring-ring">
                                    <input
                                        type="color"
                                        name=(slot.field)
                                        value=((slot.get)(theme).as_str())
                                        data-variable=(slot.variable)
                                        class="size-7 shrink-0 cursor-pointer rounded-base border-2 border-border bg-transparent p-0 [&::-webkit-color-swatch]:rounded-[calc(var(--border-radius)-2px)] [&::-webkit-color-swatch]:border-0 [&::-webkit-color-swatch-wrapper]:p-0"
                                    >
                                    <span class="flex-1">(slot.label)</span>
                                    <code data-hex="" class="text-xs">((slot.get)(theme).as_str())</code>
                                </label>
                            }
                        </div>
                    )
                    setting(title: "Font",
                        native_select(attrs: attributes! { name="font" class="w-full" },
                            for font in FONTS {
                                native_select_option(
                                    attrs: attributes! {
                                        value=(font.name)
                                        data-href=(font.href())
                                        selected=(font.name == theme.font.name)
                                    },
                                    (font.name)
                                )
                            }
                        )
                    )
                    option_group(title: "Border radius", name: "r", columns: 4, options: &radii)
                    option_group(title: "Horizontal box shadow", name: "x", columns: 5, options: &shadow_x)
                    option_group(title: "Vertical box shadow", name: "y", columns: 5, options: &shadow_y)
                    option_group(title: "Heading font weight", name: "hw", columns: 3, options: &heading)
                    option_group(title: "Base font weight", name: "bw", columns: 3, options: &base)
                </form>
                sheet_footer(
                    sheet_close(attrs: attributes! { class=(button_variants(ButtonVariant::Default, Default::default())) }, "Save changes")
                    button(variant: ButtonVariant::Neutral, attrs: attributes! { type="button" onclick=(RESET_SCRIPT) }, "Reset")
                )
            )
        )
    })
}

/// The palettes, as swatch radios. Only the selected color mode's palettes
/// show: each swatch hides itself unless the form's checked `mode` matches.
#[component]
async fn presets() -> Result<impl View> {
    Ok(view! {
        <div class="grid grid-cols-2 gap-2">
            for preset in PRESETS {
                <label
                    class=(if preset.mode == Mode::Monochromatic {
                        "group-has-[[name=mode][value=duotone]:checked]/theme:hidden"
                    } else {
                        "group-has-[[name=mode][value=monochromatic]:checked]/theme:hidden"
                    })
                >
                    <input
                        class="peer sr-only"
                        type="radio"
                        name="preset"
                        value=(format!("{}-{}", preset.mode.as_str(), preset.name))
                        data-colors=(preset_colors(preset))
                    >
                    <span class="flex h-9 cursor-pointer items-center gap-2 rounded-base border-2 border-border bg-secondary-background px-2 text-sm font-base capitalize text-foreground peer-checked:bg-main peer-checked:text-main-foreground peer-focus-visible:ring-2 peer-focus-visible:ring-ring">
                        <span class="flex shrink-0 overflow-hidden rounded-base border-2 border-border">
                            <span class="size-4" style=(format!("background-color: {}", preset.main))></span>
                            <span class="w-0.5 shrink-0 bg-border"></span>
                            <span class="size-4" style=(format!("background-color: {}", preset.background))></span>
                        </span>
                        (preset.name)
                    </span>
                </label>
            }
        </div>
    })
}

/// A preset's colors as the JSON the theme script copies into the form.
fn preset_colors(preset: &crate::theme::Preset) -> String {
    let charts = preset
        .charts
        .iter()
        .enumerate()
        .map(|(index, color)| format!(r#""c{}":"{color}""#, index + 1))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        r#"{{"main":"{}","bg":"{}",{charts}}}"#,
        preset.main, preset.background
    )
}

/// A labelled block in the sheet.
#[component]
async fn setting(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <fieldset class="grid gap-3">
            <legend class="mb-3 text-sm font-heading">(title)</legend>
            (child)
        </fieldset>
    })
}

/// A row of option buttons for one numeric setting: `(value, label, checked)`.
#[component]
async fn option_group(
    title: &str,
    name: &str,
    columns: u8,
    options: &[(String, String, bool)],
) -> Result<impl View> {
    let grid = match columns {
        3 => "grid grid-cols-3 gap-2",
        4 => "grid grid-cols-4 gap-2",
        _ => "grid grid-cols-5 gap-2",
    };
    Ok(view! {
        setting(title: title,
            <div class=(grid)>
                for (value, text, checked) in options {
                    <label class=(OPTION)>
                        <input class="sr-only" type="radio" name=(name) value=(value.as_str()) checked=(*checked)>
                        (text.as_str())
                    </label>
                }
            </div>
        )
    })
}

/// The Copy dialog: the whole `styles.css`, or just the `:root` block.
#[component]
async fn copy_dialog(stylesheet: &str, root_block: &str) -> Result<impl View> {
    Ok(view! {
        dialog(
            id: "copy-theme",
            dialog_content(
                attrs: attributes! { class="max-h-[calc(100dvh-2rem)] overflow-y-auto sm:max-w-2xl!" },
                dialog_header(
                    dialog_title("Install this theme")
                    dialog_description("For a Topcoat app with the neobrutalism components.")
                )
                <div class="group/copy grid min-w-0 gap-4">
                    <div class="grid h-10 grid-cols-2 border-2 border-border bg-secondary-background">
                        <label class="flex cursor-pointer items-center justify-center border-r-2 border-border font-base has-checked:bg-main has-checked:text-main-foreground has-focus-visible:underline">
                            <input class="sr-only" type="radio" name="copy-tab" checked="">
                            "styles.css"
                        </label>
                        <label class="flex cursor-pointer items-center justify-center font-base has-checked:bg-main has-checked:text-main-foreground has-focus-visible:underline">
                            <input class="sr-only" type="radio" name="copy-tab" data-tab="root">
                            ":root only"
                        </label>
                    </div>
                    <div class="grid min-w-0 gap-3 group-has-[[data-tab=root]:checked]/copy:hidden">
                        <p class="text-sm font-base">
                            "Save as " <code>"styles.css"</code>
                            " at your app's root, replacing the one " <code>"topcoat ui init"</code>
                            " wrote. It is the Tailwind input your " <code>"build.rs"</code> " compiles."
                        </p>
                        css_block(id: "theme-css", css: stylesheet)
                        <div class="flex gap-2">
                            copy_button(target: "theme-css")
                            <a
                                href="/styling/theme.css"
                                download="styles.css"
                                class=(button_variants(ButtonVariant::Neutral, Default::default()))
                            >
                                "Download"
                            </a>
                        </div>
                    </div>
                    <div class="hidden min-w-0 gap-3 group-has-[[data-tab=root]:checked]/copy:grid">
                        <p class="text-sm font-base">
                            "Already have the neobrutalism theme? Replace the " <code>":root"</code>
                            " block in your " <code>"styles.css"</code> " with this."
                        </p>
                        css_block(id: "root-css", css: root_block)
                        <div class="flex gap-2">copy_button(target: "root-css")</div>
                    </div>
                </div>
            )
        )
    })
}

#[component]
async fn css_block(id: &str, css: &str) -> Result<impl View> {
    Ok(view! {
        <pre class="max-h-[50dvh] min-w-0 overflow-auto rounded-base border-2 border-border bg-[#272933] p-4 text-xs leading-5 text-white"><code id=(id)>(css)</code></pre>
    })
}

/// Copies an element's text and says so on the button for a moment.
#[component]
async fn copy_button(target: &str, #[default] attrs: Attributes) -> Result<impl View> {
    let script = format!(
        "navigator.clipboard.writeText(document.getElementById('{target}').textContent); \
         this.textContent = 'Copied!'; setTimeout(() => this.textContent = 'Copy', 1500)"
    );
    Ok(view! {
        button(attrs: attributes! { type="button" onclick=(script) (attrs) }, "Copy")
    })
}
