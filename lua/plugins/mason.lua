local signs = { " ", " ", " ", " " }
vim.diagnostic.config({
	virtual_text = {
		prefix = "",
		format = function(diagnostic)
			return string.format("%s (%s)", signs[diagnostic.severity], diagnostic.source, diagnostic.message)
		end,
	},

	signs = { text = signs },
	virtual_lines = { current_line = true },
	underline = false,
	severity_sort = true,
})

return {
	{
		"mason-org/mason-lspconfig.nvim",
		opts = {
			ensure_installed = {
				"lua_ls",
			},
		},
		dependencies = {
			{ "mason-org/mason.nvim", opts = {} },
			"neovim/nvim-lspconfig",
		},
	},

	{
		"jay-babu/mason-null-ls.nvim",
		event = { "BufReadPre", "BufNewFile" },
		dependencies = {
			"mason-org/mason.nvim",
			"nvimtools/none-ls.nvim",
		},
		config = function()
			require("mason-null-ls").setup({
				automatic_installation = false,
				ensure_installed = { "stylua" },
			})
		end,
	},
}
