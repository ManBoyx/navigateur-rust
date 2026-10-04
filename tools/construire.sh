#!/usr/bin/env bash
# Compile Navigateur Rust. Il faut : Rust (cargo), et les bibliothèques de développement GTK 3 et
# WebKitGTK, par exemple sur CachyOS/Arch : sudo pacman -S gtk3 webkit2gtk-4.1
# (sur Debian/Ubuntu : sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev)
set -euo pipefail
ici="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ici"

command -v cargo >/dev/null 2>&1 || { echo "ÉCHEC : cargo (Rust) est introuvable. Installe-le : https://rustup.rs" >&2; exit 1; }
command -v pkg-config >/dev/null 2>&1 || { echo "ÉCHEC : pkg-config est introuvable (paquet pkgconf ou pkg-config)." >&2; exit 1; }
pkg-config --exists gtk+-3.0 || { echo "ÉCHEC : gtk+-3.0 introuvable (installe gtk3 / libgtk-3-dev)." >&2; exit 1; }
if ! pkg-config --exists webkit2gtk-4.1 && ! pkg-config --exists webkit2gtk-4.0; then
  echo "ÉCHEC : webkit2gtk-4.1 (ou 4.0) introuvable (installe webkit2gtk-4.1 / libwebkit2gtk-4.1-dev)." >&2
  exit 1
fi

echo "== Compilation (cargo build --release)"
cargo build --release

echo "== Terminé : target/release/navigateur-rust"
