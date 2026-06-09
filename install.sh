#!/bin/sh
set -eu

APP_NAME="kitkit-cli"
BIN_NAME="kitkit-cli"
DEFAULT_REPO="stello-agent/kitkit-cli"

REPO="${KITKIT_REPO:-$DEFAULT_REPO}"
VERSION="${KITKIT_VERSION:-latest}"
INSTALL_DIR="${KITKIT_INSTALL_DIR:-}"
TARGET="${KITKIT_TARGET:-}"

usage() {
  cat <<EOF
Install kitkit-cli from GitHub Releases.

Usage:
  install.sh [options]

Options:
  --version <version>      Release tag to install, for example v0.1.1.
                           Defaults to the latest GitHub release.
  --dir <path>             Install directory. Defaults to a writable bin
                           directory in PATH, then ~/.local/bin.
  --target <target>        Override the release target triple.
  --repo <owner/repo>      GitHub repository. Defaults to $DEFAULT_REPO.
  -h, --help               Show this help.

Environment:
  KITKIT_VERSION           Same as --version.
  KITKIT_INSTALL_DIR       Same as --dir.
  KITKIT_TARGET            Same as --target.
  KITKIT_REPO              Same as --repo.
  KITKIT_LINUX_LIBC        Override Linux libc detection: gnu or musl.
EOF
}

log() {
  printf '%s\n' "$*"
}

die() {
  printf 'kitkit-cli install: %s\n' "$*" >&2
  exit 1
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      [ "$#" -ge 2 ] || die "--version requires a value"
      VERSION="$2"
      shift 2
      ;;
    --version=*)
      VERSION="${1#*=}"
      shift
      ;;
    --dir | --install-dir)
      [ "$#" -ge 2 ] || die "$1 requires a value"
      INSTALL_DIR="$2"
      shift 2
      ;;
    --dir=* | --install-dir=*)
      INSTALL_DIR="${1#*=}"
      shift
      ;;
    --target)
      [ "$#" -ge 2 ] || die "--target requires a value"
      TARGET="$2"
      shift 2
      ;;
    --target=*)
      TARGET="${1#*=}"
      shift
      ;;
    --repo)
      [ "$#" -ge 2 ] || die "--repo requires a value"
      REPO="$2"
      shift 2
      ;;
    --repo=*)
      REPO="${1#*=}"
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die "unknown option: $1"
      ;;
  esac
done

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

fetch_stdout() {
  url="$1"
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fsSL "$url"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- "$url"
  else
    die "curl or wget is required"
  fi
}

fetch_file() {
  url="$1"
  out="$2"
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fL "$url" -o "$out"
  elif command -v wget >/dev/null 2>&1; then
    wget -q -O "$out" "$url"
  else
    die "curl or wget is required"
  fi
}

resolve_tag() {
  version="$1"
  if [ "$version" = "latest" ]; then
    tag="$(fetch_stdout "https://api.github.com/repos/$REPO/releases/latest" \
      | sed -n 's/^[[:space:]]*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' \
      | head -n 1)"
    [ -n "$tag" ] || die "could not resolve latest release tag for $REPO"
    printf '%s\n' "$tag"
    return
  fi

  case "$version" in
    v*) printf '%s\n' "$version" ;;
    *) printf 'v%s\n' "$version" ;;
  esac
}

detect_libc() {
  override="${KITKIT_LINUX_LIBC:-}"
  if [ -n "$override" ]; then
    case "$override" in
      gnu | musl) printf '%s\n' "$override"; return ;;
      *) die "KITKIT_LINUX_LIBC must be gnu or musl" ;;
    esac
  fi

  if command -v ldd >/dev/null 2>&1; then
    if ldd --version 2>&1 | grep -qi musl; then
      printf '%s\n' musl
      return
    fi
    if ldd --version 2>&1 | grep -qi 'glibc\|gnu libc'; then
      printf '%s\n' gnu
      return
    fi
  fi

  if command -v getconf >/dev/null 2>&1 && getconf GNU_LIBC_VERSION >/dev/null 2>&1; then
    printf '%s\n' gnu
    return
  fi

  printf '%s\n' musl
}

detect_target() {
  if [ -n "$TARGET" ]; then
    printf '%s\n' "$TARGET"
    return
  fi

  os="$(uname -s)"
  machine="$(uname -m)"

  case "$machine" in
    x86_64 | amd64) arch="x86_64" ;;
    arm64 | aarch64) arch="aarch64" ;;
    *) die "unsupported CPU architecture: $machine" ;;
  esac

  case "$os" in
    Darwin)
      printf '%s-apple-darwin\n' "$arch"
      ;;
    Linux)
      if [ "$arch" = "aarch64" ]; then
        printf '%s-unknown-linux-musl\n' "$arch"
      else
        printf '%s-unknown-linux-%s\n' "$arch" "$(detect_libc)"
      fi
      ;;
    *)
      die "unsupported OS: $os. Use install.ps1 on Windows."
      ;;
  esac
}

make_tmpdir() {
  base="${TMPDIR:-/tmp}"
  mktemp -d "$base/kitkit-cli.XXXXXX" 2>/dev/null || mktemp -d -t kitkit-cli
}

path_has_dir() {
  dir="$1"
  case ":${PATH:-}:" in
    *":$dir:"*) return 0 ;;
    *) return 1 ;;
  esac
}

shell_quote() {
  printf "'%s'" "$(printf '%s' "$1" | sed "s/'/'\\\\''/g")"
}

dir_writable_or_creatable() {
  dir="$1"
  if [ -d "$dir" ]; then
    [ -w "$dir" ]
    return
  fi
  parent="$(dirname "$dir")"
  [ -d "$parent" ] && [ -w "$parent" ]
}

choose_install_dir() {
  if [ -n "$INSTALL_DIR" ]; then
    printf '%s\n' "$INSTALL_DIR"
    return
  fi

  [ -n "${HOME:-}" ] || die "HOME is not set; pass --dir to choose an install directory"

  for dir in "$HOME/.local/bin" "$HOME/bin" /opt/homebrew/bin /usr/local/bin; do
    if path_has_dir "$dir" && dir_writable_or_creatable "$dir"; then
      printf '%s\n' "$dir"
      return
    fi
  done

  for dir in /usr/local/bin /opt/homebrew/bin "$HOME/.local/bin" "$HOME/bin"; do
    if dir_writable_or_creatable "$dir"; then
      printf '%s\n' "$dir"
      return
    fi
  done

  printf '%s\n' "$HOME/.local/bin"
}

install_binary() {
  src="$1"
  dest="$2"
  dest_dir="$(dirname "$dest")"

  if [ ! -d "$dest_dir" ]; then
    if ! mkdir -p "$dest_dir" 2>/dev/null; then
      command -v sudo >/dev/null 2>&1 || die "cannot create $dest_dir; rerun with --dir or install sudo"
      sudo mkdir -p "$dest_dir"
    fi
  fi

  if [ -w "$dest_dir" ]; then
    if command -v install >/dev/null 2>&1; then
      install -m 0755 "$src" "$dest"
    else
      cp "$src" "$dest"
      chmod 0755 "$dest"
    fi
  else
    command -v sudo >/dev/null 2>&1 || die "cannot write to $dest_dir; rerun with --dir or install sudo"
    if command -v install >/dev/null 2>&1; then
      sudo install -m 0755 "$src" "$dest"
    else
      sudo cp "$src" "$dest"
      sudo chmod 0755 "$dest"
    fi
  fi
}

need_cmd uname
need_cmd tar
need_cmd find
need_cmd grep
need_cmd sed

TAG="$(resolve_tag "$VERSION")"
TARGET="$(detect_target)"
ARCHIVE="$APP_NAME-$TAG-$TARGET.tar.gz"
BASE_URL="https://github.com/$REPO/releases/download/$TAG"

TMPDIR_INSTALL="$(make_tmpdir)"
cleanup() {
  rm -rf "$TMPDIR_INSTALL"
}
trap cleanup EXIT
trap 'cleanup; exit 1' HUP INT TERM

ARCHIVE_PATH="$TMPDIR_INSTALL/$ARCHIVE"
EXTRACT_DIR="$TMPDIR_INSTALL/extract"
mkdir -p "$EXTRACT_DIR"

log "Installing $APP_NAME $TAG for $TARGET"
fetch_file "$BASE_URL/$ARCHIVE" "$ARCHIVE_PATH"

tar -xzf "$ARCHIVE_PATH" -C "$EXTRACT_DIR"
BINARY_PATH="$(find "$EXTRACT_DIR" -type f -name "$BIN_NAME" | head -n 1)"
[ -n "$BINARY_PATH" ] || die "$BIN_NAME was not found in $ARCHIVE"
chmod 0755 "$BINARY_PATH"

INSTALL_DIR="$(choose_install_dir)"
DEST="$INSTALL_DIR/$BIN_NAME"
install_binary "$BINARY_PATH" "$DEST"


if "$DEST" --version >/dev/null 2>&1; then
  INSTALLED_VERSION="$("$DEST" --version 2>/dev/null || true)"
  log "Installed $INSTALLED_VERSION to $DEST"
else
  log "Installed $APP_NAME to $DEST"
fi

if ! path_has_dir "$INSTALL_DIR"; then
  log "Add $INSTALL_DIR to PATH to run $BIN_NAME without the full path."
fi

DEST_QUOTED="$(shell_quote "$DEST")"
REMOVE_CMD="rm -f"
if [ ! -w "$INSTALL_DIR" ]; then
  REMOVE_CMD="sudo rm -f"
fi

log ""
log "To uninstall:"
log "  $DEST_QUOTED auth logout  # optional: remove cached tokens first"
log "  $REMOVE_CMD $DEST_QUOTED"
