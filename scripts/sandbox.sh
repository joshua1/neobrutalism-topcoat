#!/usr/bin/env bash
# Builds a scratch Topcoat app that mounts the given component sources (and
# their docs-site examples, when present) straight from this registry, so a
# component can be compile-checked on its own before it is registered.
#
# Usage: scripts/sandbox.sh <sandbox-dir> <component>...
#   e.g. scripts/sandbox.sh /tmp/sb-dialog button dialog alert_dialog
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIR="$1"
shift
mkdir -p "$DIR/src"
NAME="sandbox-$(basename "$DIR" | tr -c 'a-z0-9' '-' | sed 's/-*$//')"

cat > "$DIR/Cargo.toml" <<EOF
[package]
name = "$NAME"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
topcoat = { version = "0.10.0", features = ["icon-iconify", "runtime"] }

[build-dependencies]
topcoat = { version = "0.10.0", default-features = false, features = ["icon-iconify"] }
EOF

cat > "$DIR/build.rs" <<'EOF'
fn main() {
    topcoat::icon::iconify::BuildConfig::new().icon_set("lucide").stage().unwrap();
}
EOF

cat > "$DIR/src/main.rs" <<'EOF'
#![allow(dead_code)]
mod components;
mod examples;
fn main() {}
EOF

: > "$DIR/src/components.rs"
: > "$DIR/src/examples.rs"
for c in "$@"; do
  echo "#[path = \"$ROOT/src/components/$c.rs\"] pub mod $c;" >> "$DIR/src/components.rs"
  if [ -f "$ROOT/site/src/examples/$c.rs" ]; then
    echo "#[path = \"$ROOT/site/src/examples/$c.rs\"] pub mod $c;" >> "$DIR/src/examples.rs"
  fi
done

export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target/sandbox}"
cargo build --quiet --manifest-path "$DIR/Cargo.toml"
