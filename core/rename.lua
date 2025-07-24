-- 🧘 Zen-Vim: File Rename Module
-- In-place file renaming functionality

local M = {}

function M.rename_current_file()
  local current_file = vim.fn.expand("%:p")
  
  if current_file == "" then
    vim.notify("No file to rename", vim.log.levels.WARN)
    return
  end
  
  local current_dir = vim.fn.fnamemodify(current_file, ":h")
  local current_name = vim.fn.fnamemodify(current_file, ":t")
  
  -- Prompt for new filename
  vim.ui.input({
    prompt = "Rename to: ",
    default = current_name,
    completion = "file",
  }, function(new_name)
    if not new_name or new_name == "" or new_name == current_name then
      return
    end
    
    local new_file = current_dir .. "/" .. new_name
    
    -- Check if new file already exists
    if vim.fn.filereadable(new_file) == 1 then
      vim.notify("File already exists: " .. new_name, vim.log.levels.ERROR)
      return
    end
    
    -- Save current buffer if modified
    if vim.bo.modified then
      vim.cmd("write")
    end
    
    -- Rename the file
    local success = vim.fn.rename(current_file, new_file)
    if success == 0 then
      -- Update buffer name and reload
      vim.cmd("edit " .. vim.fn.fnameescape(new_file))
      vim.cmd("bdelete " .. vim.fn.fnameescape(current_file))
      vim.notify("Renamed: " .. current_name .. " → " .. new_name, vim.log.levels.INFO)
    else
      vim.notify("Failed to rename file", vim.log.levels.ERROR)
    end
  end)
end

return M 