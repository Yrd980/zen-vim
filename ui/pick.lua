-- 🧘 Zen-Vim: Picker UI Module
-- Wrapper for snack.nvim pickers with zen aesthetics

local M = {}

-- Check if snack is available
local function has_snack()
  return pcall(require, "snack")
end

-- Fallback picker using vim.ui.select for files
local function fallback_files()
  local files = vim.fn.split(vim.fn.glob("**/*"), "\n")
  -- Filter out directories
  files = vim.tbl_filter(function(file)
    return vim.fn.isdirectory(file) == 0
  end, files)
  
  vim.ui.select(files, {
    prompt = "Select file:",
    format_item = function(item)
      return item
    end,
  }, function(choice)
    if choice then
      vim.cmd("edit " .. vim.fn.fnameescape(choice))
    end
  end)
end

-- Fallback picker for buffers
local function fallback_buffers()
  local buffers = {}
  for _, buf in ipairs(vim.api.nvim_list_bufs()) do
    if vim.api.nvim_buf_is_loaded(buf) and vim.bo[buf].buflisted then
      local name = vim.api.nvim_buf_get_name(buf)
      if name ~= "" then
        table.insert(buffers, {
          buf = buf,
          name = vim.fn.fnamemodify(name, ":~:."),
        })
      end
    end
  end
  
  vim.ui.select(buffers, {
    prompt = "Select buffer:",
    format_item = function(item)
      return item.name
    end,
  }, function(choice)
    if choice then
      vim.api.nvim_set_current_buf(choice.buf)
    end
  end)
end

-- Fallback picker for grep (basic)
local function fallback_grep()
  vim.ui.input({
    prompt = "Grep for: ",
  }, function(pattern)
    if pattern and pattern ~= "" then
      vim.cmd("vimgrep /" .. vim.fn.escape(pattern, "/") .. "/j **/*")
      vim.cmd("copen")
    end
  end)
end

-- File picker
function M.files()
  if has_snack() then
    require("snack").picker.files()
  else
    fallback_files()
  end
end

-- Text grep picker
function M.grep()
  if has_snack() then
    require("snack").picker.grep()
  else
    fallback_grep()
  end
end

-- Buffer picker
function M.buffers()
  if has_snack() then
    require("snack").picker.buffers()
  else
    fallback_buffers()
  end
end

-- Live grep with initial pattern
function M.grep_word()
  local word = vim.fn.expand("<cword>")
  if has_snack() then
    require("snack").picker.grep({ search = word })
  else
    vim.cmd("vimgrep /" .. vim.fn.escape(word, "/") .. "/j **/*")
    vim.cmd("copen")
  end
end

-- Recent files picker
function M.recent()
  if has_snack() then
    require("snack").picker.recent()
  else
    vim.notify("Recent files picker requires snack.nvim", vim.log.levels.WARN)
  end
end

-- Git files picker
function M.git_files()
  if has_snack() then
    require("snack").picker.git_files()
  else
    -- Simple git ls-files fallback
    local files = vim.fn.systemlist("git ls-files")
    if vim.v.shell_error == 0 then
      vim.ui.select(files, {
        prompt = "Git files:",
      }, function(choice)
        if choice then
          vim.cmd("edit " .. vim.fn.fnameescape(choice))
        end
      end)
    else
      vim.notify("Not in a git repository", vim.log.levels.WARN)
    end
  end
end

return M 