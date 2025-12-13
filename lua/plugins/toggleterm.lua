return {
	"akinsho/toggleterm.nvim",
	version = "*",
	opts = { --[[ things you want to change go here]]
		start_in_insert = false,
		open_mapping = "ESC",
	},
	config = function()
		local runner_term = require("toggleterm.terminal").Terminal:new({
			cmd = "zsh",
			hidden = false,
			direction = "float",
		})

		vim.keymap.set({ "n" }, "<leader>tt", function()
			runner_term.direction = "float"
			runner_term:toggle()
		end, { desc = "FLoat Terminal" })

		vim.keymap.set({ "n" }, "<leader>tv", function()
			runner_term.direction = "vertical"
			runner_term:toggle()
		end, { desc = "Vertical Terminal" })

		vim.keymap.set({ "n" }, "<leader>th", function()
			runner_term.direction = "horizontal"
			runner_term:toggle()
		end, { desc = "Horizontal Terminal" })
	end,
}
