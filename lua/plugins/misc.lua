return {
	{
		"numToStr/Comment.nvim",
		opts = {
			-- add any options here
		},
	},
	{
		"kylechui/nvim-surround",
		version = "^3.0.0", -- Use for stability; omit to use `main` branch for the latest features
		event = "VeryLazy",
		config = function()
			require("nvim-surround").setup({
				-- Configuration here, or leave empty to use defaults
			})
		end,
	},
	{
		"folke/todo-comments.nvim",
		dependencies = { "nvim-lua/plenary.nvim" },
		opts = {
			-- your configuration comes here
			-- or leave it empty to use the default settings
			-- refer to the configuration section below
		},
	},
	{
		"catgoose/nvim-colorizer.lua",
		event = "BufReadPre",
		opts = { -- set to setup table
		},
	},
	{
		"sphamba/smear-cursor.nvim",
		opts = {},
	},
	{
		"karb94/neoscroll.nvim",
		opts = {},
	},
	{
		"windwp/nvim-autopairs",
		event = "InsertEnter",
		config = true,
		-- use opts = {} for passing setup options
		-- this is equivalent to setup({}) function
	},
	{
		"mcauley-penney/visual-whitespace.nvim",
		event = "ModeChanged *:[vV\22]", -- optionally, lazy load on entering visual mode
		opts = {
			enabled = true,
			highlight = { link = "Visual", default = true },
			match_types = {
				space = true,
				tab = true,
				nbsp = true,
				lead = false,
				trail = false,
			},
			list_chars = {
				space = "·",
				tab = "↦",
				nbsp = "␣",
				lead = "‹",
				trail = "›",
			},
			fileformat_chars = {
				unix = "↲",
				mac = "←",
				dos = "↙",
			},
			ignore = { filetypes = {}, buftypes = {} },
		},
	},
}
