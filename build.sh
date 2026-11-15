#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

if ! command -v rustup >/dev/null 2>&1; then
  if [ ! -f "$HOME/.cargo/env" ]; then
    curl --proto '=https' --tlsv1.2 -fsS https://sh.rustup.rs | sh -s -- -y --profile minimal
  fi
  . "$HOME/.cargo/env"
fi

rustup target add wasm32-unknown-unknown
node tools/build.mjs
