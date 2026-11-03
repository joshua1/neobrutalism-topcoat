fn main() {
    // Stage the Lucide icon set for the `iconify_icon!` references in the
    // `#[cfg(test)]` component sources. Gated behind `stage-icons` so apps
    // that depend on this registry crate never download anything.
    #[cfg(feature = "stage-icons")]
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("lucide")
        .stage()
        .unwrap();
}
