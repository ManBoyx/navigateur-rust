#!/usr/bin/env bash
# Fabrique Navigateur-Rust-x86_64.AppImage à partir du binaire déjà compilé (tools/construire.sh).
# Il faut mksquashfs 4.4+ (compression zstd). GTK 3 et WebKitGTK ne sont PAS embarqués dans l'AppImage
# (les embarquer, avec toutes leurs propres dépendances, ferait un fichier de plusieurs centaines de Mo
# et fragile aux mises à jour de sécurité) : ils doivent déjà être installés sur le système, comme pour
# n'importe quelle appli GTK/WebKitGTK (GNOME Web, par exemple). Sur CachyOS : sudo pacman -S gtk3 webkit2gtk-4.1
set -euo pipefail
ici="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ici"

BIN="target/release/navigateur-rust"
SORTIE="Navigateur-Rust-x86_64.AppImage"
RUNTIME_TAG=20251108
RUNTIME_SHA256=2fca8b443c92510f1483a883f60061ad09b46b978b2631c807cd873a47ec260d

[ -x "$BIN" ] || { echo "ÉCHEC : $BIN est introuvable. Lance d'abord tools/construire.sh" >&2; exit 1; }
mksquashfs_bin="${MKSQUASHFS:-$(command -v mksquashfs || true)}"
[ -x "$mksquashfs_bin" ] || { echo "ÉCHEC : mksquashfs est introuvable (paquet squashfs-tools)." >&2; exit 1; }
"$mksquashfs_bin" -help 2>&1 | grep -qi zstd || { echo "ÉCHEC : ce mksquashfs ne sait pas compresser en zstd." >&2; exit 1; }

mkdir -p cache/appimage
runtime="cache/appimage/runtime-x86_64"
[ -f "$runtime" ] || curl -sfL -o "$runtime" "https://github.com/AppImage/type2-runtime/releases/download/$RUNTIME_TAG/runtime-x86_64"
echo "$RUNTIME_SHA256  $runtime" | sha256sum -c - >/dev/null || { echo "ÉCHEC : la somme du lanceur AppImage est incorrecte" >&2; exit 1; }

echo "== AppDir"
appdir="AppDir"
rm -rf -- "$appdir/usr"
mkdir -p "$appdir/usr/bin"
cp "$BIN" "$appdir/usr/bin/navigateur-rust"
cp packaging/AppRun "$appdir/AppRun"
cp packaging/navigateur-rust.desktop "$appdir/navigateur-rust.desktop"
cp packaging/navigateur-rust.svg "$appdir/navigateur-rust.svg"
ln -sf navigateur-rust.svg "$appdir/.DirIcon"
chmod +x "$appdir/AppRun" "$appdir/usr/bin/navigateur-rust"

echo "== Image compressée"
rm -f -- appimage.squashfs "$SORTIE"
"$mksquashfs_bin" "$appdir" appimage.squashfs -root-owned -noappend -comp zstd -Xcompression-level 19 -b 1M -no-progress >/dev/null
cat "$runtime" appimage.squashfs > "$SORTIE"
chmod +x "$SORTIE"
rm -f appimage.squashfs
sha256sum "$SORTIE" > "$SORTIE.sha256"
ls -la "$SORTIE"
