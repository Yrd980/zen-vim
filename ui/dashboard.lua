-- 🧘 Zen-Vim: Dashboard Module
-- ASCII dashboard with zen aesthetics

local M = {}

local dashboard_art = {
  "                                                                     ",
  "       ████████╗███████╗███╗   ██╗      ██╗   ██╗██╗███╗   ███╗     ",
  "       ╚══██╔══╝██╔════╝████╗  ██║      ██║   ██║██║████╗ ████║     ",
  "          ██║   █████╗  ██╔██╗ ██║█████╗██║   ██║██║██╔████╔██║     ",
  "          ██║   ██╔══╝  ██║╚██╗██║╚════╝╚██╗ ██╔╝██║██║╚██╔╝██║     ",
  "          ██║   ███████╗██║ ╚████║       ╚████╔╝ ██║██║ ╚═╝ ██║     ",
  "          ╚═╝   ╚══════╝╚═╝  ╚═══╝        ╚═══╝  ╚═╝╚═╝     ╚═╝     ",
  "                                                                     ",
  "                    🧘 Minimalist • Fast • Zen                      ",
  "                                                                     ",
}

local menu_items = {
  { key = "pf", desc = "Find Files", action = "require('ui.pick').files()" },
  { key = "pt", desc = "Grep Text", action = "require('ui.pick').grep()" },
  { key = "pb", desc = "Buffers", action = "require('ui.pick').buffers()" },
  { key = "pr", desc = "Resume Session", action = "require('core.resume').resume_last()" },
  { key = "q", desc = "Quit", action = "qa" },
}

function M.show()
  -- Create a new buffer
  local buf = vim.api.nvim_create_buf(false, true)
  
  -- Set buffer options
  vim.api.nvim_buf_set_option(buf, "modifiable", false)
  vim.api.nvim_buf_set_option(buf, "buflisted", false)
  vim.api.nvim_buf_set_option(buf, "bufhidden", "wipe")
  vim.api.nvim_buf_set_option(buf, "filetype", "dashboard")
  
  -- Create content
  local content = {}
  
  -- Add some top padding
  for i = 1, 3 do
    table.insert(content, "")
  end
  
  -- Add ASCII art
  for _, line in ipairs(dashboard_art) do
    table.insert(content, line)
  end
  
  -- Add separator
  table.insert(content, "")
  table.insert(content, "")
  
  -- Add menu items
  for _, item in ipairs(menu_items) do
    local line = string.format("    [%s]  %s", item.key, item.desc)
    table.insert(content, line)
  end
  
  -- Add bottom info
  table.insert(content, "")
  table.insert(content, "")
  table.insert(content, "    Leader key: <space>")
  table.insert(content, "    Press any menu key to continue...")
  
  -- Set content
  vim.api.nvim_buf_set_lines(buf, 0, -1, false, content)
  
  -- Open in current window
  vim.api.nvim_set_current_buf(buf)
  
  -- Center the content
  vim.cmd("normal! gg")
  vim.cmd("normal! zz")
  
  -- Set up keymaps for menu items
  for _, item in ipairs(menu_items) do
    vim.keymap.set("n", item.key, function()
      if item.action:match("^require") then
        -- Lua function call
        local func = load("return " .. item.action)()
        func()
      else
        -- Vim command
        vim.cmd(item.action)
      end
    end, { buffer = buf, nowait = true })
  end
  
  -- Close dashboard on any file open
  vim.api.nvim_create_autocmd("BufEnter", {
    callback = function(args)
      if args.buf ~= buf and vim.bo[args.buf].buflisted then
        if vim.api.nvim_buf_is_valid(buf) then
          vim.api.nvim_buf_delete(buf, { force = true })
        end
        return true -- Remove autocmd
      end
    end,
  })
end

return M 