return {
	{ "folke/lazy.nvim", version = "*" },
	{
		"folke/lazydev.nvim",
		ft = "lua", -- only load on lua files
		opts = {
			library = {
				-- See the configuration section for more details
				-- Load luvit types when the `vim.uv` word is found
				{ path = "${3rd}/luv/library", words = { "vim%.uv" } },
			},
		},
	},
	{
		"folke/noice.nvim",
		event = "VeryLazy",
		opts = {
			-- add any options here
		},
		dependencies = {
			-- if you lazy-load any plugin below, make sure to add proper `module="..."` entries
			"MunifTanjim/nui.nvim",
			-- OPTIONAL:
			--   `nvim-notify` is only needed, if you want to use the notification view.
			--   If not available, we use `mini` as the fallback
			"rcarriga/nvim-notify",
		},
	},
	{
		"folke/snacks.nvim",
		priority = 1000,
		lazy = false,
		opts = {
			image = {},
			indent = {},
			dashboard = require("config.dashboard"),
			terminal = {
				keys = {
					["<Esc>"] = function()
						vim.cmd("stopinsert")
					end,
				},
			},
		},
		config = function(_, opts)
			local notify = vim.notify
			require("snacks").setup(opts)

			vim.notify = notify
		end,
	},
}
