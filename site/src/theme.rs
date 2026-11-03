//! The theme generator.
//!
//! A [`Theme`] is every value a neobrutalism theme can change: the colors,
//! the corner radius, the hard shadow's offset, the font weights and the
//! font. The styling page edits one in the browser and stores it in the
//! `neo-theme` cookie; the server reads that cookie back on every request,
//! so each page renders with the visitor's theme, and [`Theme::stylesheet`]
//! produces the `styles.css` a Topcoat app installs.

use std::fmt;

/// The cookie that holds the visitor's theme.
pub const COOKIE: &str = "neo-theme";

/// The registry's theme file. [`Theme::stylesheet`] swaps in a new `:root`
/// block and keeps everything else, so the generated CSS always matches
/// what `topcoat ui add` components expect.
const THEME_CSS: &str = include_str!("../../themes/neobrutalism.css");

/// A color as `#rrggbb`, the only form `<input type="color">` produces.
///
/// The constructor validates its input, so a `Hex` is always well-formed:
/// a value read from a cookie (user input) can't inject CSS.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hex(String);

impl Hex {
    /// Parses `#rrggbb` or `rrggbb` (any case). Anything else is `None`.
    pub fn parse(value: &str) -> Option<Hex> {
        let digits = value.strip_prefix('#').unwrap_or(value);
        if digits.len() == 6 && digits.chars().all(|c| c.is_ascii_hexdigit()) {
            Some(Hex(format!("#{}", digits.to_ascii_lowercase())))
        } else {
            None
        }
    }

    /// For the hard-coded defaults and presets below, which are known to be
    /// valid. Panics otherwise, which a test would catch.
    fn known(value: &str) -> Hex {
        Hex::parse(value).expect("a valid built-in color")
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// `format!("{}", hex)` and `(hex)` in a view print `#rrggbb`.
impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Which list of starting palettes the picker shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    /// The main color on a light tint of itself.
    #[default]
    Monochromatic,
    /// The main color on a contrasting background.
    Duotone,
}

impl Mode {
    pub const ALL: [Mode; 2] = [Mode::Monochromatic, Mode::Duotone];

    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Monochromatic => "monochromatic",
            Mode::Duotone => "duotone",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Mode::Monochromatic => "Monochromatic",
            Mode::Duotone => "Duotone",
        }
    }

    fn parse(value: &str) -> Option<Mode> {
        Mode::ALL.into_iter().find(|mode| mode.as_str() == value)
    }
}

/// A ready-made palette: a starting point the visitor can then change.
/// From the Octane site's `src/data/colors.ts`.
pub struct Preset {
    pub name: &'static str,
    pub mode: Mode,
    pub main: &'static str,
    pub background: &'static str,
    pub charts: [&'static str; 5],
}

pub const PRESETS: &[Preset] = &[
    Preset { name: "red", mode: Mode::Monochromatic, main: "#ff6164", background: "#ffe0e0", charts: ["#ff6164", "#5294ff", "#facc00", "#05e17a", "#7a83ff"] },
    Preset { name: "orange", mode: Mode::Monochromatic, main: "#ff7a05", background: "#ffedd6", charts: ["#ff7a05", "#0099ff", "#ffbf00", "#00d696", "#7a83ff"] },
    Preset { name: "amber", mode: Mode::Monochromatic, main: "#ffbf00", background: "#fef3c8", charts: ["#ffbf00", "#0099ff", "#ff7a05", "#00d696", "#7a83ff"] },
    Preset { name: "yellow", mode: Mode::Monochromatic, main: "#facc00", background: "#fdf7c4", charts: ["#facc00", "#7a83ff", "#ff4d50", "#00d696", "#0099ff"] },
    Preset { name: "lime", mode: Mode::Monochromatic, main: "#8ae600", background: "#e7f7cf", charts: ["#8ae500", "#0099ff", "#ff4d50", "#facc00", "#7a83ff"] },
    Preset { name: "green", mode: Mode::Monochromatic, main: "#05e17a", background: "#defce9", charts: ["#05e17a", "#ff7a05", "#0099ff", "#ffbf00", "#7a83ff"] },
    Preset { name: "emerald", mode: Mode::Monochromatic, main: "#00d696", background: "#d0fbea", charts: ["#00d696", "#7a83ff", "#facc00", "#ff4d50", "#0099ff"] },
    Preset { name: "teal", mode: Mode::Monochromatic, main: "#00d6bd", background: "#ccfaf2", charts: ["#00d6bd", "#0099ff", "#7a83ff", "#ff4d50", "#facc00"] },
    Preset { name: "cyan", mode: Mode::Monochromatic, main: "#00c8f0", background: "#cdf6fe", charts: ["#00c8f0", "#ff7a05", "#7a83ff", "#ff4d50", "#facc00"] },
    Preset { name: "sky", mode: Mode::Monochromatic, main: "#0099ff", background: "#ddeffd", charts: ["#0099ff", "#ff4d50", "#facc00", "#05e17a", "#7a83ff"] },
    Preset { name: "blue", mode: Mode::Monochromatic, main: "#5294ff", background: "#dcebfe", charts: ["#5294ff", "#ff4d50", "#facc00", "#05e17a", "#7a83ff"] },
    Preset { name: "indigo", mode: Mode::Monochromatic, main: "#858dff", background: "#e0e2ff", charts: ["#858dff", "#facc00", "#ff4d50", "#00d696", "#0099ff"] },
    Preset { name: "violet", mode: Mode::Monochromatic, main: "#a985ff", background: "#eee6fe", charts: ["#a985ff", "#00d696", "#facc00", "#ff4d50", "#0099ff"] },
    Preset { name: "purple", mode: Mode::Monochromatic, main: "#ca7aff", background: "#f2e5ff", charts: ["#ca7aff", "#ffbf00", "#00d696", "#ff7a05", "#0099ff"] },
    Preset { name: "fuchsia", mode: Mode::Monochromatic, main: "#e96bff", background: "#fbe5ff", charts: ["#e96bff", "#facc00", "#ff7a05", "#ff4d50", "#7a83ff"] },
    Preset { name: "pink", mode: Mode::Monochromatic, main: "#fc64ab", background: "#fce9f3", charts: ["#fc64ab", "#facc00", "#ff7a05", "#ff4d50", "#7a83ff"] },
    Preset { name: "rose", mode: Mode::Monochromatic, main: "#ff6678", background: "#ffe0e3", charts: ["#ff6678", "#7a83ff", "#facc00", "#00d696", "#5294ff"] },
    Preset { name: "red", mode: Mode::Duotone, main: "#ff6164", background: "#99d6ff", charts: ["#ff6164", "#0099ff", "#facc00", "#05e17a", "#7a83ff"] },
    Preset { name: "orange", mode: Mode::Duotone, main: "#ff7a05", background: "#99d6ff", charts: ["#ff7a05", "#0099ff", "#ffbf00", "#00d696", "#7a83ff"] },
    Preset { name: "amber", mode: Mode::Duotone, main: "#ffbf00", background: "#ffc999", charts: ["#ffbf00", "#ff7a05", "#0099ff", "#00d696", "#7a83ff"] },
    Preset { name: "yellow", mode: Mode::Duotone, main: "#facc00", background: "#99a0ff", charts: ["#facc00", "#7a83ff", "#ff4d50", "#00d696", "#0099ff"] },
    Preset { name: "lime", mode: Mode::Duotone, main: "#8ae600", background: "#99d6ff", charts: ["#8ae500", "#0099ff", "#ff4d50", "#facc00", "#7a83ff"] },
    Preset { name: "green", mode: Mode::Duotone, main: "#05e17a", background: "#ffc999", charts: ["#05e17a", "#ff7a05", "#0099ff", "#ffbf00", "#7a83ff"] },
    Preset { name: "emerald", mode: Mode::Duotone, main: "#00d696", background: "#99a0ff", charts: ["#00d696", "#7a83ff", "#facc00", "#ff4d50", "#0099ff"] },
    Preset { name: "teal", mode: Mode::Duotone, main: "#00d6bd", background: "#99d6ff", charts: ["#00d6bd", "#0099ff", "#7a83ff", "#ff4d50", "#facc00"] },
    Preset { name: "cyan", mode: Mode::Duotone, main: "#00c8f0", background: "#ffc999", charts: ["#00c8f0", "#ff7a05", "#7a83ff", "#ff4d50", "#facc00"] },
    Preset { name: "sky", mode: Mode::Duotone, main: "#0099ff", background: "#99a0ff", charts: ["#0099ff", "#7a83ff", "#facc00", "#05e17a", "#ff4d50"] },
    Preset { name: "violet", mode: Mode::Duotone, main: "#a985ff", background: "#99ffe0", charts: ["#a985ff", "#00d696", "#facc00", "#ff4d50", "#0099ff"] },
    Preset { name: "fuchsia", mode: Mode::Duotone, main: "#e96bff", background: "#ffec99", charts: ["#e96bff", "#facc00", "#ff7a05", "#ff4d50", "#7a83ff"] },
    Preset { name: "rose", mode: Mode::Duotone, main: "#ff6678", background: "#99a0ff", charts: ["#ff6678", "#7a83ff", "#facc00", "#00d696", "#5294ff"] },
];

/// A Google font the picker offers, with the weights Google serves for it
/// (asking for a weight a font lacks makes Google reject the whole request).
#[derive(Debug, PartialEq, Eq)]
pub struct Font {
    pub name: &'static str,
    pub weights: &'static [u16],
}

const ALL_WEIGHTS: &[u16] = &[400, 500, 600, 700, 800, 900];

pub const FONTS: &[Font] = &[
    Font { name: "DM Sans", weights: ALL_WEIGHTS },
    Font { name: "Archivo", weights: ALL_WEIGHTS },
    Font { name: "Archivo Black", weights: &[400] },
    Font { name: "Bricolage Grotesque", weights: &[400, 500, 600, 700, 800] },
    Font { name: "Chivo", weights: ALL_WEIGHTS },
    Font { name: "Geist", weights: ALL_WEIGHTS },
    Font { name: "IBM Plex Sans", weights: &[400, 500, 600, 700] },
    Font { name: "Inter", weights: ALL_WEIGHTS },
    Font { name: "Instrument Sans", weights: &[400, 500, 600, 700] },
    Font { name: "Lexend", weights: ALL_WEIGHTS },
    Font { name: "Manrope", weights: &[400, 500, 600, 700, 800] },
    Font { name: "Outfit", weights: ALL_WEIGHTS },
    Font { name: "Poppins", weights: ALL_WEIGHTS },
    Font { name: "Public Sans", weights: ALL_WEIGHTS },
    Font { name: "Rubik", weights: ALL_WEIGHTS },
    Font { name: "Sora", weights: &[400, 500, 600, 700, 800] },
    Font { name: "Space Grotesk", weights: &[400, 500, 600, 700] },
    Font { name: "Space Mono", weights: &[400, 700] },
    Font { name: "Syne", weights: &[400, 500, 600, 700, 800] },
    Font { name: "Work Sans", weights: ALL_WEIGHTS },
];

impl Font {
    /// The Google Fonts stylesheet URL for this font and its weights.
    pub fn href(&self) -> String {
        let family = self.name.replace(' ', "+");
        let weights: Vec<String> = self.weights.iter().map(u16::to_string).collect();
        format!(
            "https://fonts.googleapis.com/css2?family={family}:wght@{}&display=swap",
            weights.join(";")
        )
    }

    fn find(name: &str) -> Option<&'static Font> {
        FONTS.iter().find(|font| font.name.eq_ignore_ascii_case(name))
    }
}

/// The choices the picker offers for the numeric settings (the same ones
/// as the Octane site). Cookies may carry any value inside the ranges that
/// [`Theme::from_cookie`] accepts.
pub const RADII: [u8; 4] = [0, 5, 10, 15];
pub const SHADOW_OFFSETS: [i8; 5] = [-4, -2, 0, 2, 4];
pub const HEADING_WEIGHTS: [u16; 3] = [700, 800, 900];
pub const BASE_WEIGHTS: [u16; 3] = [500, 600, 700];

/// One color slot the visitor can pick: the form field name, the label, the
/// CSS variable it sets, and how to read and write it on a [`Theme`].
pub struct ColorSlot {
    pub field: &'static str,
    pub label: &'static str,
    pub variable: &'static str,
    pub get: fn(&Theme) -> &Hex,
    set: fn(&mut Theme, Hex),
}

pub const COLOR_SLOTS: &[ColorSlot] = &[
    ColorSlot { field: "main", label: "Main", variable: "--main", get: |t| &t.main, set: |t, c| t.main = c },
    ColorSlot { field: "bg", label: "Background", variable: "--background", get: |t| &t.background, set: |t, c| t.background = c },
    ColorSlot { field: "bg2", label: "Secondary background", variable: "--secondary-background", get: |t| &t.secondary_background, set: |t, c| t.secondary_background = c },
    ColorSlot { field: "fg", label: "Text", variable: "--foreground", get: |t| &t.foreground, set: |t, c| t.foreground = c },
    ColorSlot { field: "mainfg", label: "Text on main", variable: "--main-foreground", get: |t| &t.main_foreground, set: |t, c| t.main_foreground = c },
    ColorSlot { field: "border", label: "Border & shadow", variable: "--border", get: |t| &t.border, set: |t, c| t.border = c },
    ColorSlot { field: "c1", label: "Chart 1", variable: "--chart-1", get: |t| &t.charts[0], set: |t, c| t.charts[0] = c },
    ColorSlot { field: "c2", label: "Chart 2", variable: "--chart-2", get: |t| &t.charts[1], set: |t, c| t.charts[1] = c },
    ColorSlot { field: "c3", label: "Chart 3", variable: "--chart-3", get: |t| &t.charts[2], set: |t, c| t.charts[2] = c },
    ColorSlot { field: "c4", label: "Chart 4", variable: "--chart-4", get: |t| &t.charts[3], set: |t, c| t.charts[3] = c },
    ColorSlot { field: "c5", label: "Chart 5", variable: "--chart-5", get: |t| &t.charts[4], set: |t, c| t.charts[4] = c },
];

/// Everything a neobrutalism theme can change.
#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub mode: Mode,
    pub main: Hex,
    pub background: Hex,
    pub secondary_background: Hex,
    pub foreground: Hex,
    pub main_foreground: Hex,
    pub border: Hex,
    pub charts: [Hex; 5],
    pub radius: u8,
    pub shadow_x: i8,
    pub shadow_y: i8,
    pub heading_weight: u16,
    pub base_weight: u16,
    pub font: &'static Font,
}

/// The registry theme's own values (the "blue" palette).
impl Default for Theme {
    fn default() -> Theme {
        Theme {
            mode: Mode::Monochromatic,
            main: Hex::known("#5294ff"),
            background: Hex::known("#dcebfe"),
            secondary_background: Hex::known("#ffffff"),
            foreground: Hex::known("#000000"),
            main_foreground: Hex::known("#000000"),
            border: Hex::known("#000000"),
            charts: ["#5294ff", "#ff4d50", "#facc00", "#05e17a", "#7a83ff"].map(Hex::known),
            radius: 5,
            shadow_x: 4,
            shadow_y: 4,
            heading_weight: 700,
            base_weight: 500,
            font: &FONTS[0],
        }
    }
}

impl Theme {
    /// Reads a theme from the cookie value written by the styling page:
    /// `key=value` pairs joined by `&`, e.g. `main=ff6164&r=10&font=Inter`.
    ///
    /// The cookie is user input, so every value is checked. Unknown keys and
    /// invalid values are ignored, and anything missing keeps its default.
    pub fn from_cookie(value: &str) -> Theme {
        let mut theme = Theme::default();
        for pair in value.split('&') {
            let Some((key, raw)) = pair.split_once('=') else { continue };
            let raw = raw.replace('+', " ");
            match key {
                "mode" => theme.mode = Mode::parse(&raw).unwrap_or(theme.mode),
                "r" => theme.radius = parse_in(&raw, 0..=40).unwrap_or(theme.radius),
                "x" => theme.shadow_x = parse_in(&raw, -12..=12).unwrap_or(theme.shadow_x),
                "y" => theme.shadow_y = parse_in(&raw, -12..=12).unwrap_or(theme.shadow_y),
                "hw" => theme.heading_weight = parse_weight(&raw).unwrap_or(theme.heading_weight),
                "bw" => theme.base_weight = parse_weight(&raw).unwrap_or(theme.base_weight),
                "font" => theme.font = Font::find(&raw).unwrap_or(theme.font),
                _ => {
                    // A color slot, if the key names one and the value is a color.
                    let slot = COLOR_SLOTS.iter().find(|slot| slot.field == key);
                    if let (Some(slot), Some(color)) = (slot, Hex::parse(&raw)) {
                        (slot.set)(&mut theme, color);
                    }
                }
            }
        }
        theme
    }

    /// The `:root` block: every token the components read.
    pub fn root_block(&self) -> String {
        let (x, y) = (self.shadow_x, self.shadow_y);
        let mut css = String::from(":root {\n");
        let mut line = |name: &str, value: String| css.push_str(&format!("  {name}: {value};\n"));

        line("--border-radius", format!("{}px", self.radius));
        line("--box-shadow-x", format!("{x}px"));
        line("--box-shadow-y", format!("{y}px"));
        line("--reverse-box-shadow-x", format!("{}px", -x));
        line("--reverse-box-shadow-y", format!("{}px", -y));
        line("--heading-font-weight", self.heading_weight.to_string());
        line("--base-font-weight", self.base_weight.to_string());
        line("--base-font-family", format!("\"{}\", sans-serif", self.font.name));
        line("--background", self.background.to_string());
        line("--secondary-background", self.secondary_background.to_string());
        line("--foreground", self.foreground.to_string());
        line("--main-foreground", self.main_foreground.to_string());
        line("--main", self.main.to_string());
        line("--border", self.border.to_string());
        line("--ring", self.border.to_string());
        line("--overlay", "oklch(0% 0 0 / 0.8)".to_string());
        line("--shadow", "var(--box-shadow-x) var(--box-shadow-y) 0px 0px var(--border)".to_string());
        for (index, chart) in self.charts.iter().enumerate() {
            line(&format!("--chart-{}", index + 1), chart.to_string());
        }
        line("--chart-active-dot", self.border.to_string());

        css.push('}');
        css
    }

    /// The complete `styles.css` for a Topcoat app: the registry theme with
    /// this theme's `:root` block.
    pub fn stylesheet(&self) -> String {
        let start = THEME_CSS.find(":root {").expect("the theme has a :root block");
        let end = start + THEME_CSS[start..].find("\n}").expect("the :root block closes") + 2;
        let font_note = if self.font.name == FONTS[0].name {
            String::new()
        } else {
            format!(
                "/* Load the font in your layout's <head>:\n   <link rel=\"stylesheet\" href=\"{}\"> */\n",
                self.font.href()
            )
        };
        format!("{}{font_note}{}{}", &THEME_CSS[..start], self.root_block(), &THEME_CSS[end..])
    }

    /// The cookie value for this theme; the inverse of [`Theme::from_cookie`].
    ///
    /// The styling page's script writes the cookie in the browser, so only the
    /// tests call this; it documents the format and proves the round trip.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn to_cookie(&self) -> String {
        let mut pairs = vec![format!("mode={}", self.mode.as_str())];
        for slot in COLOR_SLOTS {
            pairs.push(format!("{}={}", slot.field, &(slot.get)(self).as_str()[1..]));
        }
        pairs.push(format!("r={}", self.radius));
        pairs.push(format!("x={}", self.shadow_x));
        pairs.push(format!("y={}", self.shadow_y));
        pairs.push(format!("hw={}", self.heading_weight));
        pairs.push(format!("bw={}", self.base_weight));
        pairs.push(format!("font={}", self.font.name.replace(' ', "+")));
        pairs.join("&")
    }
}

/// Parses an integer and keeps it only if it is inside `range`.
fn parse_in<T>(raw: &str, range: std::ops::RangeInclusive<T>) -> Option<T>
where
    T: std::str::FromStr + PartialOrd,
{
    raw.parse().ok().filter(|value| range.contains(value))
}

/// A CSS font weight: 100 to 900 in steps of 100.
fn parse_weight(raw: &str) -> Option<u16> {
    parse_in(raw, 100..=900).filter(|weight| weight % 100 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_round_trips() {
        let mut theme = Theme::default();
        theme.main = Hex::known("#ff6164");
        theme.radius = 15;
        theme.shadow_x = -2;
        theme.font = Font::find("space grotesk").unwrap();
        assert_eq!(Theme::from_cookie(&theme.to_cookie()), theme);
    }

    #[test]
    fn invalid_values_keep_defaults() {
        let theme = Theme::from_cookie("main=red;}&r=999&x=abc&hw=650&font=Comic+Sans&nope=1");
        assert_eq!(theme, Theme::default());
    }

    #[test]
    fn presets_are_valid_colors() {
        for preset in PRESETS {
            for color in [preset.main, preset.background].iter().chain(&preset.charts) {
                assert!(Hex::parse(color).is_some(), "{}: {color}", preset.name);
            }
        }
    }

    #[test]
    fn stylesheet_replaces_only_the_root_block() {
        let css = Theme::default().stylesheet();
        assert_eq!(css.matches(":root {").count(), 1);
        assert!(css.contains("@theme inline"));
        assert!(css.contains("--main: #5294ff;"));
    }
}
