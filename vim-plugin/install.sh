#!/bin/bash

# Zellij Playbooks Vim Plugin Installation Script

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Installing Zellij Playbooks Vim Plugin...${NC}"

# Determine Vim runtime directory
if [ -d "$HOME/.vim" ]; then
    VIM_DIR="$HOME/.vim"
elif [ -d "$HOME/.config/nvim" ]; then
    VIM_DIR="$HOME/.config/nvim"
    echo -e "${YELLOW}Detected Neovim configuration directory${NC}"
else
    VIM_DIR="$HOME/.vim"
    echo -e "${YELLOW}Creating Vim directory: $VIM_DIR${NC}"
    mkdir -p "$VIM_DIR"
fi

# Create necessary directories
echo -e "${GREEN}Creating plugin directories...${NC}"
mkdir -p "$VIM_DIR/plugin"
mkdir -p "$VIM_DIR/autoload"

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Copy plugin files
echo -e "${GREEN}Copying plugin files...${NC}"
cp "$SCRIPT_DIR/plugin/zellij-playbooks.vim" "$VIM_DIR/plugin/"
cp "$SCRIPT_DIR/autoload/zellij_playbooks.vim" "$VIM_DIR/autoload/"

# Check if zellij is available
if command -v zellij &> /dev/null; then
    echo -e "${GREEN}✓ Zellij is installed and available${NC}"
    echo -e "${GREEN}Zellij version: $(zellij --version)${NC}"
else
    echo -e "${RED}⚠ Warning: Zellij command not found in PATH${NC}"
    echo -e "${YELLOW}Please make sure Zellij is installed and available in your PATH${NC}"
fi

# Check if zellij pipe command is available
if zellij --help 2>&1 | grep -q "pipe"; then
    echo -e "${GREEN}✓ Zellij pipe command is available${NC}"
else
    echo -e "${RED}⚠ Warning: 'zellij pipe' command not found${NC}"
    echo -e "${YELLOW}Please make sure you have a recent version of Zellij with pipe support${NC}"
fi

echo -e "${GREEN}Installation completed successfully!${NC}"
echo ""
echo -e "${YELLOW}Usage:${NC}"
echo "  - Send word under cursor: <leader>zp (default)"
echo "  - Send current line: <leader>zl (default)"
echo "  - Send selected text: <leader>zv (default)"
echo "  - Show help: :ZellijPlaybooksHelp"
echo ""
echo -e "${YELLOW}To customize keybindings, add to your .vimrc:${NC}"
echo "  let g:zellij_playbooks_keybinding = '<your-key>'"
echo "  let g:zellij_playbooks_line_keybinding = '<your-key>'"
echo "  let g:zellij_playbooks_visual_keybinding = '<your-key>'"
echo ""
echo -e "${GREEN}Restart Vim/Neovim to load the plugin.${NC}"