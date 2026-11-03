//! The documentation site for the neobrutalism Topcoat components.
//!
//! `src/components/` is installed from the registry one directory up with
//! `topcoat ui add --all --registry neobrutalism`, exactly as an app would.
//! Run `scripts/sync.sh` after changing a component or an example.

mod app;
mod catalog;
mod components;
mod examples;
mod styling_demos;
mod theme;

#[tokio::main]
async fn main() {
    topcoat::start(app::router()).await.unwrap();
}
