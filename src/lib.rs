//! Neobrutalism components for Topcoat UI.
//!
//! This crate is a topcoat-ui registry: `topcoat ui add <name> --registry
//! neobrutalism` copies a component's source into your project. Nothing here
//! is linked into your app.
//!
//! The component sources also compile as test modules, so `cargo check-components` checks
//! every component before anyone installs it.

#[cfg(all(test, feature = "stage-icons"))]
pub mod components;
