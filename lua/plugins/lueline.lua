return {
	"nvim-lualine/lualine.nvim",
	dependencies = { "nvim-tree/nvim-web-devicons" },
	config = function()
		require("lualine").setup({
			options = {
				disabled_filetypes = {
					statusline = {},
					winbar = {},
				},
			},

			extensions = {
				"neo-tree",
				"fugitive",
			},
		})
	end,
}
