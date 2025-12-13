require("config.lazy")

vim.cmd("tnoremap <Esc> <C-\\><C-n>")
vim.cmd("set clipboard+=unnamedplus")

vim.cmd("source ~/.config/nvim/.vimrc")

vim.cmd("set laststatus=3")

vim.opt.titlestring = [[%f %h%m%r%w %{v:progname} (%{tabpagenr()} of %{tabpagenr('$')})]]
