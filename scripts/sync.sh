#!/usr/bin/env bash
# Refreshes the docs site after a component or example changes:
# reinstalls every component from the registry (as an app would) and
# regenerates the docs catalog.
#
# Usage: scripts/sync.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT/site"

if [ ! -f components.toml ]; then
  # `init` writes the built-in theme over styles.css; the site imports ours
  # instead, so keep it.
  cp styles.css styles.css.keep
  topcoat ui init
  mv styles.css.keep styles.css
fi
topcoat ui add --all --overwrite --registry neobrutalism
python3 -I "$ROOT/scripts/gen_catalog.py"
