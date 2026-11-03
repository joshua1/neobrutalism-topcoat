fn main() {
    // Tailwind scans src/ (and the theme it imports); rerun when they change.
    for path in ["src", "styles.css", "../themes", "build.rs"] {
        println!("cargo:rerun-if-changed={path}");
    }
    topcoat::tailwind::BuildConfig::new()
        .input("styles.css")
        .render()
        .unwrap();
    // The Lucide icons used by the components and examples.
    topcoat::icon::iconify::BuildConfig::new()
        .icon_set("lucide")
        .stage()
        .unwrap();
}
