//! What the docs pages show: one [`Doc`] per component, its example sections,
//! and the sources displayed under "Code" and "Installation".
//!
//! The data lives in `catalog/generated.rs`, written by
//! `scripts/gen_catalog.py` from the Octane docs (`src/markdown/components`)
//! and the examples in `site/src/examples`.

mod generated;

pub use generated::{DOCS, component_source, render_example};

/// A component's documentation page.
pub struct Doc {
    /// The URL segment, e.g. `alert-dialog`.
    pub slug: &'static str,
    /// The Rust module (and installed file) name, e.g. `alert_dialog`.
    pub module: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    /// The examples, in page order. The first one is the page's top preview.
    pub sections: &'static [Section],
}

/// One example on a docs page.
pub struct Section {
    pub heading: &'static str,
    pub body: &'static str,
    /// The example function's name in `examples::<module>`.
    pub example: &'static str,
}

/// The URL of a docs page.
pub fn doc_href(doc: &Doc) -> String {
    format!("/docs/{}", doc.slug)
}

/// The source of one example function, cut out of its examples file: from
/// its `#[component]` line up to the next one.
pub fn example_source(module: &str, example: &str) -> &'static str {
    let file = generated::example_file(module);
    let signature = format!("pub async fn {example}(");
    let Some(at) = file.find(&signature) else {
        return file;
    };
    let start = file[..at].rfind("#[component]").unwrap_or(at);
    let end = file[at..]
        .find("#[component]")
        .map_or(file.len(), |offset| at + offset);
    file[start..end].trim_end()
}
