" Zellij Playbooks – Autoload Functions
" Core functionality for sending text to Zellij via `zellij pipe`

" Sends text to Zellij using the `zellij pipe` command
function! zellij_playbooks#send_text(text)
    " Check if zellij is available
    if !zellij_playbooks#is_zellij_available()
        echoerr 'Zellij not found. Please install zellij: https://zellij.dev'
        return 0
    endif

    " Escape text for shell execution
    let escaped_text = shellescape(a:text)
    let command = 'zellij pipe --name zellij-playbooks-pipe -- ' . escaped_text
    let result = system(command)

    " Check execution result
    if v:shell_error != 0
        echoerr 'Failed to send text to Zellij: ' . trim(result)
        return 0
    else
        echo 'Text sent to Zellij successfully'
        return 1
    endif
endfunction

" Sends the current line to Zellij
function! zellij_playbooks#send_current_line()
    let current_line = getline('.')
    if empty(current_line) || current_line =~ '^\s*$'
        echo 'Current line is empty'
        return 0
    endif
    return zellij_playbooks#send_text(current_line)
endfunction

" Sends the word under the cursor to Zellij
function! zellij_playbooks#send_current_word()
    let current_word = expand('<cword>')
    if empty(current_word)
        echo 'No word under cursor'
        return 0
    endif
    return zellij_playbooks#send_text(current_word)
endfunction

" Sends selected text in visual mode to Zellij
function! zellij_playbooks#send_selected_text()
    " Save visual mode type (character or line-wise)
    let mode = visualmode()
    noautocmd normal! gv  " Restore last visual selection
    let [line_start, col_start] = getpos("'<")[1:2]
    let [line_end, col_end] = getpos("'>")[1:2]

    " Get all lines in the selection
    let lines = getline(line_start, line_end)

    if len(lines) == 0 || (len(lines) == 1 && empty(lines[0]))
        echo 'No text selected'
        return 0
    endif

    " Handle character-wise selection
    if mode ==# 'v'
        let lines[0] = lines[0][col_start-1 : col_end-1]
    endif

    " Handle multi-line selection
    if len(lines) > 1
        let lines[0] = lines[0][col_start-1:]
        let lines[-1] = lines[-1][:col_end-1]
    endif

    let text = join(lines, "\n")
    if empty(text) || text =~ '^\s*$'
        echo 'Selected text is empty'
        return 0
    endif

    return zellij_playbooks#send_text(text)
endfunction

" Displays help message with commands and keybindings
function! zellij_playbooks#show_help()
    echo "📌 Zellij Playbooks Plugin — Help"
    echo ""
    echo "🔧 Commands:"
    echo "  :ZellijPlaybooks <text>       – Send arbitrary text to Zellij"
    echo "  :ZellijPlaybooksLine          – Send current line"
    echo "  :ZellijPlaybooksWord          – Send word under cursor"
    echo "  :ZellijPlaybooksHelp          – Show this help"
    echo ""
    echo "⌨️  Key Mappings:"
    echo "  " . get(g:, 'zellij_playbooks_keybinding', '<leader>zp') . "    – Send word under cursor"
    echo "  " . get(g:, 'zellij_playbooks_line_keybinding', '<leader>zl') . "    – Send current line"
    echo "  " . get(g:, 'zellij_playbooks_visual_keybinding', '<leader>zv') . "    – Send selected text (visual mode)"
    echo ""
    echo "⚙️  Configuration (add to your .vimrc):"
    echo "  let g:zellij_playbooks_keybinding        = '<C-p>'"
    echo "  let g:zellij_playbooks_line_keybinding   = '<C-l>'"
    echo "  let g:zellij_playbooks_visual_keybinding = '<C-v>'"
    echo ""
    echo "📦 Requires: zellij (https://zellij.dev)"
endfunction

" Checks if zellij is installed and available in PATH
function! zellij_playbooks#is_zellij_available()
    if !exists('s:zellij_available')
        let s:zellij_available = executable('zellij')
    endif
    return s:zellij_available
endfunction

" Gets zellij version (optional)
function! zellij_playbooks#get_zellij_version()
    if !zellij_playbooks#is_zellij_available()
        return 'Zellij not found'
    endif
    return substitute(system('zellij --version'), '\n$', '', 'g')
endfunction
