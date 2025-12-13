local keys = {
  {
    "tt",
    function()
      vim.cmd("Neotree toggle=true")
    end,
    desc = "Toggle File Tree",
  },
  -- file system
  {
    "<leader>f",
    group = "Finder",
  },
  {
    "<leader>ff",
    function()
      vim.cmd("Telescope find_files")
    end,
    desc = "Find Files",
  },
  {
    "<leader>fp",
    function()
      vim.cmd("Telescope live_grep")
    end,
    desc = "Find Pattern",
  },
  -- terminal
  {
    "<leader>t",
    group = "Terminal",
  },
  {
    "<leader>tt",
    function()
      vim.cmd("ToggleTerm direction=float")
    end,
    desc = "Floating Terminal",
  },
  {
    "<leader>th",
    function()
      vim.cmd("ToggleTerm direction=horizontal")
    end,
    desc = "Horizontal Terminal",
  },
  {
    "<leader>tv",
    function()
      vim.cmd("ToggleTerm direction=vertical")
    end,
    desc = "Vertical Terminal",
  },
  -- LSP
  { "<leader>l", group = "LSP" },
  {
    "<leader>lh",
    function()
      vim.cmd("Lspsaga hover_doc")
    end,
    desc = "Hover Definition",
  },
  -- misc
  {
    "<leader>q",
    function()
      vim.cmd("qa!")
    end,
    desc = "Force Quit",
  },
}

return {
  "folke/which-key.nvim",
  event = "VeryLazy",
  opts = {
    -- your configuration comes here
    -- or leave it empty to use the default settings
    -- refer to the configuration section below
    preset = "classic",
  },
  config = function()
    local wk = require("which-key")

    wk.add(keys)
  end,
}
