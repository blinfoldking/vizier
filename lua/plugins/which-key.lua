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
		desc = "Hover Definition",
	},
	-- terminal
	{
		"<leader>d",
		group = "Trouble Diagnostic",
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
