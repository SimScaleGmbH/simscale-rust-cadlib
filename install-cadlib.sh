#!/bin/sh
# Install the native cadlib library for the cadlib Rust bindings v1.0.0.
#
#   curl -fsSL https://raw.githubusercontent.com/SimScaleGmbH/simscale-rust-cadlib/v1.0.0/install-cadlib.sh | sh
#   curl -fsSL https://raw.githubusercontent.com/SimScaleGmbH/simscale-rust-cadlib/v1.0.0/install-cadlib.sh | sh -s -- --prefix "$HOME/.local"
#
# Run with --help for the options. The download is checked against the SHA-256
# digests below, generated with this release.
#
# Generated for v1.0.0. DO NOT EDIT.

set -eu

VERSION="1.0.0"
REPO="SimScaleGmbH/simscale-rust-cadlib"
BINDING="rust"
DOWNLOAD_BASE="${CADLIB_DOWNLOAD_BASE:-https://github.com/$REPO/releases/download}"

expected_sha256() {
    case "$1" in
        linux-x86_64) echo "ad078eeb06891501342e6caac573b861e322047301d6a19aa862080d922b8765" ;;
        linux-aarch64) echo "be99b98eac2daefc0dc0bc2f1b314c98f49350289c4b032b103b5fc072ebf422" ;;
        macos-aarch64) echo "d54ac1b0c4508686f20eca4475d5e26d4c99c351a57350690f6f9ab1fdd28293" ;;
    esac
}

usage() {
    cat <<EOF
Install the native cadlib library v$VERSION for the cadlib $BINDING bindings.

Usage: install-cadlib.sh [options]

  --prefix DIR     install under DIR (default: /usr/local):
                     DIR/lib/<library>, DIR/include/cadlib.h and
                     DIR/share/doc/cadlib/ (LICENSE, third-party notices)
  --dry-run        print what would be installed, change nothing
  -h, --help       show this help

The download is verified against the SHA-256 digest embedded in this script.

Environment:
  CADLIB_DOWNLOAD_BASE  download from a mirror of the release assets instead
                        of GitHub releases (the embedded digests still apply)

A prefix you cannot write to (such as the default) needs sudo:
  curl -fsSL https://raw.githubusercontent.com/$REPO/v$VERSION/install-cadlib.sh | sudo sh
Afterwards the script prints how to point your build and programs at the library.

Supported: Linux x86_64 and aarch64 (glibc), macOS on Apple silicon.
On Windows, use install-cadlib.ps1.
EOF
}

die() {
    echo "install-cadlib: $*" >&2
    exit 1
}

have() { command -v "$1" >/dev/null 2>&1; }

fetch() { # URL FILE
    if have curl; then
        curl -fsSL -o "$2" "$1"
    elif have wget; then
        wget -q -O "$2" "$1"
    else
        die "need curl or wget to download"
    fi
}

sha256_of() {
    if have sha256sum; then
        sha256sum "$1" | cut -d' ' -f1
    else
        shasum -a 256 "$1" | cut -d' ' -f1
    fi
}

# Whether DIR, or its nearest existing ancestor, is writable by this user.
can_write() {
    d=$1
    while [ ! -d "$d" ]; do d=$(dirname "$d"); done
    [ -w "$d" ]
}

prefix=/usr/local
dry_run=0
while [ $# -gt 0 ]; do
    case "$1" in
        --prefix) [ $# -ge 2 ] || die "--prefix needs a value"; prefix=$2; shift 2 ;;
        --prefix=*) prefix=${1#*=}; shift ;;
        --dry-run) dry_run=1; shift ;;
        -h | --help) usage; exit 0 ;;
        *) die "unknown option: $1 (see --help)" ;;
    esac
done
case "$prefix" in
    /*) ;;
    *) prefix="$(pwd)/$prefix" ;;
esac
libdir="$prefix/lib"

os=$(uname -s)
arch=$(uname -m)
case "$os" in
    Linux) os=linux; libfile=libcadlib.so ;;
    Darwin) os=macos; libfile=libcadlib.dylib ;;
    MINGW* | MSYS* | CYGWIN* | Windows_NT) die "on Windows, use install-cadlib.ps1 from the same release" ;;
    *) die "unsupported OS: $os (supported: Linux, macOS on Apple silicon)" ;;
esac
case "$arch" in
    x86_64 | amd64) arch=x86_64 ;;
    aarch64 | arm64) arch=aarch64 ;;
    *) die "unsupported architecture: $arch" ;;
esac
platform="$os-$arch"
if [ "$platform" = macos-x86_64 ]; then
    die "Intel Macs are not supported: cadlib is built for Apple silicon only. On an Apple silicon Mac, run this from a native (not Rosetta) shell."
fi

sha256=$(expected_sha256 "$platform")
url="$DOWNLOAD_BASE/v$VERSION/cadlib-$platform.tar.gz"

follow_up() {
    if [ "$os" = linux ] && { [ "$prefix" = /usr/local ] || [ "$prefix" = /usr ]; }; then
        echo "Next: run 'sudo ldconfig' so the linker and the loader find $libfile."
        return
    fi
    echo "Next: point your build and your programs at $libdir, e.g. in your shell profile:"
    case "$BINDING" in
        rust) printf '  export CADLIB_LIB_DIR="%s"\n' "$libdir" ;;
        go)
            if [ "$os" = macos ]; then
                printf '  export CGO_LDFLAGS="-L%s -Wl,-rpath,%s"\n' "$libdir" "$libdir"
            else
                printf '  export CGO_LDFLAGS="-L%s"\n' "$libdir"
            fi
            ;;
    esac
    case "$os" in
        linux) printf '  export LD_LIBRARY_PATH="%s${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"\n' "$libdir" ;;
        macos) printf '  export DYLD_LIBRARY_PATH="%s${DYLD_LIBRARY_PATH:+:$DYLD_LIBRARY_PATH}"\n' "$libdir" ;;
    esac
}

if [ "$dry_run" = 1 ]; then
    echo "cadlib v$VERSION for $platform (dry run: nothing is downloaded or installed)"
    echo "  download: $url"
    echo "  sha256:   $sha256"
    echo "  install:  $libdir/$libfile"
    echo "            $prefix/include/cadlib.h"
    if ! can_write "$prefix"; then
        echo "  note:     $prefix is not writable by this user; run with sudo or choose --prefix"
    fi
    follow_up
    exit 0
fi

can_write "$prefix" ||
    die "cannot write to $prefix: run with sudo ('... | sudo sh') or choose another --prefix, e.g. --prefix \"\$HOME/.local\""
have sha256sum || have shasum || die "need sha256sum or shasum to verify the download"

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
echo "Downloading cadlib v$VERSION for $platform..."
fetch "$url" "$tmp/cadlib.tar.gz" || die "download failed: $url"
actual=$(sha256_of "$tmp/cadlib.tar.gz")
[ "$actual" = "$sha256" ] ||
    die "checksum mismatch for $url: expected $sha256, got $actual. Nothing was installed."

mkdir "$tmp/x"
tar -xzf "$tmp/cadlib.tar.gz" -C "$tmp/x"
for f in "lib/$libfile" include/cadlib.h LICENSE; do
    [ -f "$tmp/x/$f" ] || die "unexpected archive layout: $f is missing"
done

docdir="$prefix/share/doc/cadlib"
mkdir -p "$libdir" "$prefix/include" "$docdir"
# Replace the library by renaming, so running programs keep their mapping of the old one.
cp "$tmp/x/lib/$libfile" "$libdir/.$libfile.new"
mv -f "$libdir/.$libfile.new" "$libdir/$libfile"
cp "$tmp/x/include/cadlib.h" "$prefix/include/cadlib.h"
cp "$tmp/x/LICENSE" "$docdir/LICENSE"
if [ -f "$tmp/x/THIRD-PARTY-NOTICES" ]; then cp "$tmp/x/THIRD-PARTY-NOTICES" "$docdir/"; fi

echo "Installed cadlib v$VERSION into $prefix ($libdir/$libfile)."
follow_up
