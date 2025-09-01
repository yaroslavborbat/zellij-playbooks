# Zellij Playbooks Vim Plugin

A Vim plugin that allows you to send selected text, current line, or word to your Zellij terminal using the `zellij pipe` command. This plugin works in conjunction with the `zellij-playbooks` plugin to provide seamless integration between Vim and Zellij.

## Features

- Send current line to Zellij
- Send selected text (visual mode) to Zellij
- Send word under cursor to Zellij
- Send specific text via command
- Configurable keybindings
- Error handling and user feedback

## Installation

### Manual Installation

```bash
./install.sh
```

### Using a Plugin Manager

#### Vim-Plug
Add to your `.vimrc`:

```vim
Plug 'path/to/zellij-playbooks/vim-plugin'
```

#### Vundle
Add to your `.vimrc`:

```vim
Plugin 'path/to/zellij-playbooks/vim-plugin'
```


## Configuration

You can customize the keybindings by setting these variables in your `.vimrc`:

```vim
" Default keybindings
let g:zellij_playbooks_keybinding = '<leader>zp'        " Send word under cursor
let g:zellij_playbooks_line_keybinding = '<leader>zl'   " Send current line
let g:zellij_playbooks_visual_keybinding = '<leader>zv' " Send selected text
```

### Example Custom Keybindings

```vim
" Use Ctrl+Enter for sending current line
let g:zellij_playbooks_line_keybinding = '<C-CR>'

" Use Ctrl+Shift+Enter for sending selected text
let g:zellij_playbooks_visual_keybinding = '<C-S-CR>'

" Use F5 for sending word under cursor
let g:zellij_playbooks_keybinding = '<F5>'
```

## Usage

### Commands

- `:ZellijPlaybooks <text>` - Send specific text to Zellij
- `:ZellijPlaybooksLine` - Send current line to Zellij
- `:ZellijPlaybooksWord` - Send word under cursor to Zellij
- `:ZellijPlaybooksHelp` - Show help information

### Key Mappings

- `<leader>zl` (default) - Send current line to Zellij
- `<leader>zp` (default) - Send word under cursor to Zellij
- `<leader>zv` (default) - Send selected text to Zellij (visual mode)

### Examples

1. **Send current line:**
   - Place cursor on any line
   - Press `<leader>zl` (or your configured keybinding)

2. **Send selected text:**
   - Select text in visual mode (v, V, or Ctrl+v)
   - Press `<leader>zv` (or your configured keybinding)

3. **Send word under cursor:**
   - Place cursor on any word
   - Press `<leader>zp` (or your configured keybinding)

4. **Send specific text:**
   - Use command: `:ZellijPlaybooks echo "Hello World"`

## Requirements

- Vim 
- Zellij terminal multiplexer
- The `zellij-playbooks` plugin loaded in Zellij

## Troubleshooting

### Zellij Command Not Found

If you get an error that `zellij` command is not found:

1. Make sure Zellij is installed and in your PATH
2. Check if the `zellij pipe` command is available in your Zellij version

### Plugin Not Working

1. Check if the plugin is loaded: `:echo g:loaded_zellij_playbooks`
2. Verify your keybindings: `:ZellijPlaybooksHelp`
3. Test the command manually: `:ZellijPlaybooks echo "test"`