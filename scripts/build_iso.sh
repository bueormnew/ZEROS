#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo build -p zeros-kernel --release
cargo run -p bap-pack -- apps/calculator/calculator.app apps/calculator/calculator.bap
cargo run -p bap-pack -- apps/clock/clock.app apps/clock/clock.bap
cargo run -p bap-pack -- apps/notepad/notepad.app apps/notepad/notepad.bap

echo "[TODO] Integrar bootloader + filesystem y generar zerOS.iso"
