return {
	{
		"nvim-tree/nvim-tree.lua",
		dependencies = {
			"nvim-tree/nvim-web-devicons",
		},
		opts = {
			filters = {
				dotfiles = false,
				custom = {},
			},
			git = {
				ignore = false,
			},
			view = {
				width = 30,
				side = "left",
			},
			renderer = {
				icons = {
					show = {
						git = true,
						folder = true,
						file = true,
						folder_arrow = true,
					},
				},
			},
		},
		config = function(_, opts)
			require("nvim-tree").setup(opts)
		end,
	},
}
