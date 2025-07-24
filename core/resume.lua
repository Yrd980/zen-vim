-- 🧘 Zen-Vim: Session Resume Module
-- Resume last session or file functionality

local M = {}

local session_file = vim.fn.stdpath("data") .. "/zenvim-session.txt"

-- Save current file to session
function M.save_current_file()
  local current_file = vim.fn.expand("%:p")
  if current_file and current_file ~= "" then
    local file = io.open(session_file, "w")
    if file then
      file:write(current_file)
      file:close()
    end
  end
end

-- Resume last file or session
function M.resume_last()
  -- Try to read last session file
  local file = io.open(session_file, "r")
  if file then
    local last_file = file:read("*line")
    file:close()
    
    if last_file and vim.fn.filereadable(last_file) == 1 then
      vim.cmd("edit " .. vim.fn.fnameescape(last_file))
      vim.notify("Resumed: " .. vim.fn.fnamemodify(last_file, ":t"), vim.log.levels.INFO)
      return
    end
  end
  
  -- Fallback: try to resume last session with mksession
  local session_path = vim.fn.stdpath("data") .. "/zenvim-session.vim"
  if vim.fn.filereadable(session_path) == 1 then
    vim.cmd("source " .. session_path)
    vim.notify("Session resumed", vim.log.levels.INFO)
    return
  end
  
  vim.notify("No session to resume", vim.log.levels.WARN)
end

-- Auto-save current file on buffer events
local function setup_auto_save()
  local group = vim.api.nvim_create_augroup("ZenVimSession", { clear = true })
  
  vim.api.nvim_create_autocmd({ "BufRead", "BufNewFile" }, {
    group = group,
    callback = function()
      M.save_current_file()
    end,
  })
  
  vim.api.nvim_create_autocmd("VimLeavePre", {
    group = group,
    callback = function()
      -- Save session on exit
      local session_path = vim.fn.stdpath("data") .. "/zenvim-session.vim"
      vim.cmd("mksession! " .. session_path)
    end,
  })
end

-- Initialize the module
setup_auto_save()

return M 