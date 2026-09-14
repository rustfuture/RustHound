#!/bin/bash
#
# RustHound install/uninstall helper.
#
# Builds the release binary from the committed lockfile, installs it to ~/.local/bin, and
# installs the bundled default rules file into the same configuration directory the
# application itself resolves.
set -euo pipefail

INSTALL_DIR="$HOME/.local/bin"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# This must match main.rs, which resolves the directory with dirs::config_dir(): that is
# ~/Library/Application Support on macOS and $XDG_CONFIG_HOME (default ~/.config) elsewhere.
# Using ~/.config on macOS would install a rules file the application never looks at.
config_dir() {
    if [ "$(uname -s)" = "Darwin" ]; then
        printf '%s' "$HOME/Library/Application Support/rusthound"
    else
        printf '%s' "${XDG_CONFIG_HOME:-$HOME/.config}/rusthound"
    fi
}

CONFIG_DIR="$(config_dir)"
RULES_FILE="$CONFIG_DIR/rules.toml"

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
    echo "Installed executable: $INSTALL_DIR/rusthound"

    mkdir -p "$CONFIG_DIR"
    if [ -f "$RULES_FILE" ]; then
        # Never overwrite rules the user has edited. Keep the bundled default beside them
        # so the difference can be inspected.
        echo "Keeping your existing rules: $RULES_FILE"
        if cp rules.toml "$RULES_FILE.new"; then
            echo "Wrote the bundled default to $RULES_FILE.new so you can diff the two."
        else
            echo "Could not write $RULES_FILE.new; your existing rules are untouched." >&2
            exit 1
        fi
    else
        if ! cp rules.toml "$RULES_FILE"; then
            echo "Could not install the default rules to $RULES_FILE." >&2
            exit 1
        fi
        echo "Installed default rules: $RULES_FILE"
    fi

    echo ""
    echo "Next steps:"
    echo "1. Edit $RULES_FILE to match your logs."
    echo "2. Make sure $INSTALL_DIR is on your PATH. If it is not, add this line to ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$PATH:$INSTALL_DIR\""
    echo "   Then run 'source ~/.bashrc' or 'source ~/.zshrc'."
    echo ""
    echo "Usage: rusthound --file /path/to/your/logfile.log --rules \"$RULES_FILE\""
    echo "Installation complete."
}

uninstall_rusthound() {
    local purge_config="$1"

    echo "Starting RustHound removal..."

    if [ -f "$INSTALL_DIR/rusthound" ]; then
        rm "$INSTALL_DIR/rusthound"
        echo "Removed executable: $INSTALL_DIR/rusthound"
    else
        echo "Executable not found: $INSTALL_DIR/rusthound"
    fi

    if [ "$purge_config" = "yes" ]; then
        if [ -d "$CONFIG_DIR" ]; then
            rm -rf "$CONFIG_DIR"
            echo "Removed configuration directory: $CONFIG_DIR"
        else
            echo "Configuration directory not found: $CONFIG_DIR"
        fi
    elif [ -d "$CONFIG_DIR" ]; then
        echo "Kept your configuration: $CONFIG_DIR"
    fi

    echo ""
    echo "Removal complete."
    if [ "$purge_config" != "yes" ]; then
        echo "Your rules were kept. To delete them too, choose the purge option."
    fi
    echo "Note: if you added $INSTALL_DIR to your PATH, remove that line from ~/.bashrc or ~/.zshrc manually."
}

echo "RustHound install/uninstall menu"
echo "--------------------------------"
echo "1. Install RustHound"
echo "2. Uninstall RustHound (keep configuration)"
echo "3. Uninstall RustHound and delete configuration"
echo "4. Exit"
echo "--------------------------------"

read -r -p "Choose an option (1-4): " choice

case "$choice" in
    1)
        install_rusthound
        ;;
    2)
        uninstall_rusthound no
        ;;
    3)
        read -r -p "Delete $CONFIG_DIR as well? This cannot be undone. (y/N): " confirm
        case "$confirm" in
            y|Y) uninstall_rusthound yes ;;
            *) echo "Cancelled." ;;
        esac
        ;;
    4)
        echo "Exiting."
        ;;
    *)
        echo "Invalid choice. Enter 1, 2, 3, or 4." >&2
        exit 1
        ;;
esac
