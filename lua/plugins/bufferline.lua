return {
	"akinsho/bufferline.nvim",
	config = function()
		require("bufferline").setup({
			options = {
				-- separator_style = "padded_slant",
				diagnostics = "nvim_lsp",

				diagnostics_indicator = function(count, level, _, _)
					local icon = level:match("error") and " " or " "
					return " " .. icon .. count
				end,

				custom_areas = {
					right = function()
						local result = {}
						local seve = vim.diagnostic.severity
						local error = #vim.diagnostic.get(0, { severity = seve.ERROR })
						local warning = #vim.diagnostic.get(0, { severity = seve.WARN })
						local info = #vim.diagnostic.get(0, { severity = seve.INFO })
						local hint = #vim.diagnostic.get(0, { severity = seve.HINT })

						if error ~= 0 then
							table.insert(result, { text = "   " .. error, guifg = "#EC5241" })
						end

						if warning ~= 0 then
							table.insert(result, { text = "   " .. warning, guifg = "#EFB839" })
						end

						if hint ~= 0 then
							table.insert(result, { text = "   " .. hint, guifg = "#A3BA5E" })
						end

						if info ~= 0 then
							table.insert(result, { text = "  " .. info, guifg = "#7EA9A7" })
						end
						return result
					end,
				},
			},
		})
	end,
}
