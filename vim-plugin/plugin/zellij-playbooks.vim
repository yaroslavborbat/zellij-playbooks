" Zellij Playbooks Vim Plugin
" Sends selected text, current line, word, or paragraph to Zellij using `zellij pipe`

" Prevent multiple loading
if exists('g:loaded_zellij_playbooks')
    finish
endif
let g:loaded_zellij_playbooks = 1

" Default keybindings
if !exists('g:zellij_playbooks_keybinding')
    let g:zellij_playbooks_keybinding = '<leader>zp'
endif

if !exists('g:zellij_playbooks_line_keybinding')
    let g:zellij_playbooks_line_keybinding = '<leader>zl'
endif

if !exists('g:zellij_playbooks_visual_keybinding')
    let g:zellij_playbooks_visual_keybinding = '<leader>zv'
endif

" Define user commands
command! -nargs=1 ZellijPlaybooks call zellij_playbooks#send_text(<q-args>)
command! ZellijPlaybooksLine call zellij_playbooks#send_current_line()
command! ZellijPlaybooksWord call zellij_playbooks#send_current_word()
command! ZellijPlaybooksHelp call zellij_playbooks#show_help()

" Normal mode mappings
execute 'nnoremap <silent> ' . g:zellij_playbooks_keybinding . ' :<C-u>call zellij_playbooks#send_current_word()<CR>'
execute 'nnoremap <silent> ' . g:zellij_playbooks_line_keybinding . ' :<C-u>call zellij_playbooks#send_current_line()<CR>'

" Visual mode mapping
execute 'xnoremap <silent> ' . g:zellij_playbooks_visual_keybinding . ' :<C-u>call zellij_playbooks#send_selected_text()<CR>'