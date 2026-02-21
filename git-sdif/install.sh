#!/bin/bash
# Installation script for git-sdif with bash completion

set -e

echo "Installing git-sdif..."

# Change to workspace root (parent directory)
cd "$(dirname "$0")/.."

# Build and install the binary
echo "Building git-sdif (release mode)..."
cargo build --release -p git-sdif

# Copy binary to a location in PATH
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

echo "Installing binary to $INSTALL_DIR..."
cp target/release/git-sdif "$INSTALL_DIR/"

# Make sure it's executable
chmod +x "$INSTALL_DIR/git-sdif"

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  WARNING: $HOME/.local/bin is not in your PATH"
    echo "Add this to your ~/.bashrc or ~/.bash_profile:"
    echo "export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
fi

# The completion script is automatically installed by the build script
# Check if bashrc needs updating for completion loading
BASHRC="$HOME/.bashrc"
if [ -f "$BASHRC" ]; then
    if ! grep -q ".bash_completion.d" "$BASHRC"; then
        echo ""
        echo "📝 To enable bash completion, add this to your ~/.bashrc:"
        echo ""
        echo "# Load custom bash completions"
        echo "if [ -d ~/.bash_completion.d ]; then"
        echo "    for file in ~/.bash_completion.d/*; do"
        echo "        [ -r \"\$file\" ] && source \"\$file\""
        echo "    done"
        echo "fi"
        echo ""
        echo "Then run: source ~/.bashrc"
    else
        echo "✅ Bash completion loading is already configured"
    fi
else
    echo "⚠️  ~/.bashrc not found - you may need to manually configure completion loading"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "Usage:"
echo "  git-sdif <branch1> [branch2]    # Compare branches"
echo "  git sdif <branch1> [branch2]    # Same as above (git subcommand)"
echo ""
echo "The completion script provides tab completion for branch names."
echo "Restart your shell or run 'source ~/.bashrc' to enable completions."
