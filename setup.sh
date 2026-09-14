#!/bin/bash
#
# RustHound install/uninstall helper.
#
# Builds the release binary from the committed lockfile, installs it to
# ~/.local/bin, and installs the default rules file to ~/.config/rusthound.
set -euo pipefail

INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.config/rusthound"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

install_rusthound() {
    echo "Starting RustHound installation..."

    if ! command -v cargo >/dev/null 2>&1; then
        echo "Rust was not found. Install it from https://rustup.rs/ and try again." >&2
        exit 1
    fi

    echo "Rust detected. Building the project..."
    cd "$REPO_ROOT"
    if ! cargo build --locked --release; then
        echo "Build failed. Fix the errors above and try again." >&2
        exit 1
    fi

    echo "Build finished. Installing the executable..."
    mkdir -p "$INSTALL_DIR"
    if ! cp target/release/rusthound "$INSTALL_DIR/"; then
        echo "Could not copy the executable. Check permissions on $INSTALL_DIR." >&2
        exit 1
    fi

    echo "RustHound installed to $INSTALL_DIR."

    mkdir -p "$CONFIG_DIR"
    if ! cp rules.toml "$CONFIG_DIR/rules.toml"; then
        echo "Could not copy rules.toml. Copy it manually: cp rules.toml $CONFIG_DIR/rules.toml" >&2
    fi

    echo ""
    echo "Next steps:"
    echo "1. The default rules.toml is now at $CONFIG_DIR/rules.toml. Edit it to match your logs."
    echo "2. Make sure $INSTALL_DIR is on your PATH. If it is not, add this line to ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
    echo "   Then run 'source ~/.bashrc' or 'source ~/.zshrc'."
    echo ""
    echo "Usage: rusthound --file /path/to/your/logfile.log --rules $CONFIG_DIR/rules.toml"
    echo "Installation complete."
}

uninstall_rusthound() {
    echo "Starting RustHound removal..."

    if [ -f "$INSTALL_DIR/rusthound" ]; then
        rm "$INSTALL_DIR/rusthound"
        echo "Removed executable: $INSTALL_DIR/rusthound"
    else
        echo "Executable not found: $INSTALL_DIR/rusthound"
    fi

    if [ -d "$CONFIG_DIR" ]; then
        rm -rf "$CONFIG_DIR"
        echo "Removed configuration directory: $CONFIG_DIR"
    else
        echo "Configuration directory not found: $CONFIG_DIR"
    fi

    echo ""
    echo "Removal complete."
    echo "Note: if you added $INSTALL_DIR to your PATH, remove that line from ~/.bashrc or ~/.zshrc manually."
}

echo "RustHound install/uninstall menu"
echo "--------------------------------"
echo "1. Install RustHound"
echo "2. Uninstall RustHound"
echo "3. Exit"
echo "--------------------------------"

read -r -p "Choose an option (1-3): " choice

case "$choice" in
    1)
        install_rusthound
        ;;
    2)
        uninstall_rusthound
        ;;
    3)
        echo "Exiting."
        ;;
    *)
        echo "Invalid choice. Enter 1, 2, or 3." >&2
        exit 1
        ;;
esac
