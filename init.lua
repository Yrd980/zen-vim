-- 🧘 Zen-Vim: Minimalist Neovim Configuration
-- Entry point for zen-vim setup

-- Disable default Neovim UI elements for clean minimal look
vim.opt.number = false
vim.opt.relativenumber = false
vim.opt.signcolumn = "no"
vim.opt.ruler = false
vim.opt.showmode = false
vim.opt.showcmd = false
vim.opt.laststatus = 0
vim.opt.cmdheight = 1

-- Essential editor settings
vim.opt.termguicolors = true
vim.opt.background = "dark"
vim.opt.splitbelow = true
vim.opt.splitright = true
vim.opt.wrap = false
vim.opt.expandtab = true
vim.opt.shiftwidth = 2
vim.opt.tabstop = 2
vim.opt.ignorecase = true
vim.opt.smartcase = true

-- Leader key configuration
vim.g.mapleader = " "
vim.g.maplocalleader = " "

-- Bootstrap lazy.nvim if not present
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.rtp:prepend(lazypath)

-- Load plugins
require("plugins")

-- Load keymaps after plugins
require("keymaps")

-- Load dashboard on startup if no files provided
vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    if vim.fn.argc() == 0 and vim.fn.line2byte("$") == -1 then
      require("ui.dashboard").show()
    end
  end,
}) 