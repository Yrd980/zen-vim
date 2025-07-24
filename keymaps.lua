-- 🧘 Zen-Vim: Keymap Configuration
-- Leader key mappings for core functionality

local keymap = vim.keymap.set

-- Leader key is space (configured in init.lua)

-- File operations
keymap("n", "<leader>pf", function()
  require("ui.pick").files()
end, { desc = "Find Files" })

keymap("n", "<leader>pt", function()
  require("ui.pick").grep()
end, { desc = "Grep Text" })

keymap("n", "<leader>pb", function()
  require("ui.pick").buffers()
end, { desc = "Buffers" })

-- Resume operations
keymap("n", "<leader>pr", function()
  require("core.resume").resume_last()
end, { desc = "Resume Last File/Session" })

-- File rename
keymap("n", "<leader>rn", function()
  require("core.rename").rename_current_file()
end, { desc = "Rename Current File" })

-- Dashboard
keymap("n", "<leader>d", function()
  require("ui.dashboard").show()
end, { desc = "Show Dashboard" })

-- Quick save and quit
keymap("n", "<leader>w", ":w<CR>", { desc = "Save File" })
keymap("n", "<leader>q", ":q<CR>", { desc = "Quit" })
keymap("n", "<leader>x", ":x<CR>", { desc = "Save and Quit" })

-- Buffer navigation
keymap("n", "<leader>n", ":bnext<CR>", { desc = "Next Buffer" })
keymap("n", "<leader>p", ":bprevious<CR>", { desc = "Previous Buffer" })
keymap("n", "<leader>c", ":bdelete<CR>", { desc = "Close Buffer" })

-- Clear search highlighting
keymap("n", "<Esc>", ":nohlsearch<CR>", { desc = "Clear Search Highlight" }) 