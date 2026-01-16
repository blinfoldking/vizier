return {
	"vyfor/cord.nvim",
	build = ":Cord update",
	opts = {
		display = {
			view = "asset",
		},
	},

	config = function(_, opts)
		require("cord").setup(opts)
	end,
}
