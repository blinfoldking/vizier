return {
	"NickvanDyke/opencode.nvim",
	dependencies = {
		-- Recommended for `ask()` and `select()`.
		-- Required for `snacks` provider.
		---@module 'snacks' <- Loads `snacks.nvim` types for configuration intellisense.
		{ "folke/snacks.nvim", opts = { input = {}, picker = {}, terminal = {} } },
		{ "akinsho/toggleterm.nvim" },
	},
	config = function()
		vim.keymap.set({ "n", "x" }, "<leader>aa", function()
			require("opencode").toggle()
		end, { desc = "Toggle Opencode Terminal" })

		---@type opencode.Opts
		vim.g.opencode_opts = {}

		-- Required for `opts.events.reload`.
		vim.o.autoread = true

		-- Recommended/example keymaps.
		vim.keymap.set({ "n", "x" }, "<leader>as", function()
			require("opencode").ask("@this: ", { submit = true })
		end, { desc = "Ask opencode" })
		vim.keymap.set({ "n", "x" }, "<leader>ax", function()
			require("opencode").select()
		end, { desc = "Execute opencode action…" })
		vim.keymap.set({ "n", "x" }, "<leader>ap", function()
			require("opencode").prompt("@this")
		end, { desc = "Add to opencode" })
		vim.keymap.set("n", "<S-C-u>", function()
			require("opencode").command("session.half.page.up")
		end, { desc = "opencode half page up" })
		vim.keymap.set("n", "<S-C-d>", function()
			require("opencode").command("session.half.page.down")
		end, { desc = "opencode half page down" })
	end,
}
