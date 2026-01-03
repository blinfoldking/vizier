vim.api.nvim_create_autocmd({ "BufEnter", "BufWinEnter" }, {
	pattern = { "*.norg" },
	command = "set conceallevel=3",
})

return {
	"nvim-neorg/neorg",
	lazy = false, -- Disable lazy loading as some `lazy.nvim` distributions set `lazy = true` by default
	version = "*", -- Pin Neorg to the latest stable release
	config = true,
	opts = {
		load = {
			["core.defaults"] = {},
			["core.concealer"] = {},
			["core.journal"] = {
				config = {
					strategy = "flat",
				},
			},
			["core.export"] = {},
			["core.export.markdown"] = {},
			["core.summary"] = {},
			["core.completion"] = {
				config = {
					engine = "nvim-cmp",
				},
			},
			["core.integrations.nvim-cmp"] = {},
			["core.integrations.telescope"] = {},
			["core.esupports.metagen"] = {
				config = {
					-- Ensure auto_update_date is true (it usually is by default)
					update_date = true,
					timezone = "utc",
					type = "auto",
				},
			},
			["core.esupports.hop"] = {},
			["core.integrations.treesitter"] = {},
			["core.esupports.indent"] = {},
		},
	},
	dependencies = { { "nvim-lua/plenary.nvim" }, { "nvim-neorg/neorg-telescope" } },
}
