-- 🧘 Zen-Vim: Plugin Configuration
-- Minimal plugin setup with snack.nvim for pickers

require("lazy").setup({
  {
    "folke/snack.nvim",
    priority = 1000,
    lazy = false,
    opts = {
      picker = {
        enabled = true,
        win = {
          input = {
            border = "rounded",
          },
          list = {
            border = "rounded",
          },
        },
      },
      dashboard = {
        enabled = false, -- We use our own dashboard
      },
    },
    keys = {
      { "<leader>pf", function() Snack.picker.files() end, desc = "Find Files" },
      { "<leader>pt", function() Snack.picker.grep() end, desc = "Grep Text" },
      { "<leader>pb", function() Snack.picker.buffers() end, desc = "Buffers" },
    },
  },
}, {
  ui = {
    border = "rounded",
  },
  checker = {
    enabled = false,
  },
  change_detection = {
    notify = false,
  },
}) 