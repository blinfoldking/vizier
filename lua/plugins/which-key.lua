local buf = vim.api.nvim_create_buf(false, true)
local keys = {
	{
		"tt",
		function()
			vim.cmd("NvimTreeToggle")
		end,
		desc = "Toggle File Tree",
	},
	-- file system
	{
		"<leader>f",
		group = "Finder",
	},
	{
		"<leader>ff",
		function()
			vim.cmd("Telescope find_files")
		end,
		desc = "Find Files",
	},
	{
		"<leader>fp",
		function()
			vim.cmd("Telescope live_grep")
		end,
		desc = "Find Pattern",
	},
	{
		"<leader>a",
		group = "AI",
	},
	-- terminal
	{
		"<leader>t",
		group = "Terminal",
	},
	-- LSP
	{ "<leader>l", group = "Code and LSP" },
	{
		"<leader>lh",
		function()
			vim.cmd("Lspsaga hover_doc")
		end,
		desc = "Hover",
	},
	{
		"<leader>lr",
		function()
			vim.cmd("Lspsaga rename")
		end,
		desc = "Rename",
	},
	{
		"<leader>lf",
		function()
			vim.cmd("Lspsaga finder")
		end,
		desc = "Find",
	},
	{
		"<leader>lca",
		function()
			vim.cmd("Lspsaga code_action")
		end,
		desc = "Code Action",
	},
	{
		"<leader>ld",
		function()
			vim.cmd("Lspsaga goto_definition")
		end,
		desc = "Goto Definition",
	},
	-- terminal
	{
		"<leader>d",
		group = "Trouble Diagnostic",
	},
	-- Minimap
	{
		"<leader>m",
		group = "Minimap",
	},
	-- Config
	{
		"<leader>c",
		group = "Config",
	},
	{
		"<leader>cc",
		function()
			if vim.g.vimrc_float_win and vim.api.nvim_win_is_valid(vim.g.vimrc_float_win) then
				vim.api.nvim_win_close(vim.g.vimrc_float_win, true)
				vim.g.vimrc_float_win = nil
			else
				vim.api.nvim_buf_call(buf, function()
					vim.cmd("edit " .. vim.fn.stdpath("config") .. "/.vimrc")
				end)
				local width = math.floor(vim.o.columns * 0.8)
				local height = math.floor(vim.o.lines * 0.8)
				local row = math.floor((vim.o.lines - height) / 2)
				local col = math.floor((vim.o.columns - width) / 2)
				local opts = {
					relative = "editor",
					width = width,
					height = height,
					row = row,
					col = col,
					style = "minimal",
					border = "rounded",
				}
				vim.g.vimrc_float_win = vim.api.nvim_open_win(buf, true, opts)
			end
		end,
		desc = "Open and Edit .vimrc",
	},
	-- Neorg
	{
		"<leader>n",
		group = "Neorg",
	},
	{
		"<leader>nc",
		function()
			vim.cmd("Neorg toggle-concealer")
		end,
		desc = "Toogle Concealer",
	},
	{
		"<leader>njj",
		function()
			vim.cmd("Neorg journal")
		end,
		desc = "Journal",
	},
	{
		"<leader>njt",
		function()
			vim.cmd("Neorg journal today")
		end,
		desc = "Daily Journal",
	},
	{
		"<leader>nt",
		group = "Telescope",
	},
	{
		"<leader>ntb",
		function()
			vim.cmd("Telescope neorg find_backlinks")
		end,
		desc = "Backlinks",
	},
	{
		"<leader>ntl",
		function()
			vim.cmd("Telescope neorg find_linkable")
		end,
		desc = "Linkables",
	},
	-- Quit
	{
		"<leader>q",
		group = "Quit",
	},
	{
		"<leader>qq",
		function()
			vim.cmd("qa!")
		end,
		desc = "Force Quit",
	},
	{
		"<leader>qw",
		function()
			vim.cmd("wqa!")
		end,
		desc = "Save and Quit",
	},
}

return {
	"folke/which-key.nvim",
	event = "VeryLazy",
	config = function()
		local wk = require("which-key")
		wk.setup({
			preset = "modern",
		})

		wk.add(keys)
	end,
}
