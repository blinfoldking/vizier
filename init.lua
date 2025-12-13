require("config.lazy")

vim.cmd("tnoremap <Esc> <C-\\><C-n>")
vim.cmd("set clipboard+=unnamedplus")

vim.cmd("source ~/.config/nvim/.vimrc")

vim.cmd("set laststatus=3")

vim.opt.titlestring = [[%f %h%m%r%w %{v:progname} (%{tabpagenr()} of %{tabpagenr('$')})]]

-- Dynamic terminal title updates for kitty
local function set_kitty_title(title)
	if vim.fn.exists("$KITTY_WINDOW_ID") == 1 then
		io.write("\x1b]2; Vizier - " .. title .. "\x07")
		io.flush()
	end
end

vim.api.nvim_create_autocmd({ "BufEnter", "WinEnter" }, {
	callback = function()
		local filename = vim.fn.expand("%:t")
		local dirname = vim.fn.fnamemodify(vim.fn.getcwd(), ":t")
		local title = filename ~= "" and (dirname .. "/" .. filename) or dirname
		set_kitty_title(title)
	end,
})

vim.api.nvim_create_autocmd("TermEnter", {
	callback = function()
		local dirname = vim.fn.fnamemodify(vim.fn.getcwd(), ":t")
		set_kitty_title(dirname .. " - Terminal")
	end,
})

vim.api.nvim_create_autocmd("DirChanged", {
	callback = function()
		local dirname = vim.fn.fnamemodify(vim.fn.getcwd(), ":t")
		set_kitty_title(dirname)
	end,
})
