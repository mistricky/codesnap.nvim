local codesnap = require("codesnap")
local static = require("codesnap.static")
local client = require("codesnap.client")

-- snap code
vim.api.nvim_create_user_command("CodeSnap", function() end, {})

vim.api.nvim_create_user_command("CodeSnapPreviewOn", function() end, {})

vim.api.nvim_create_user_command("CodeSnapPreviewOff", function() end, {})

vim.api.nvim_create_autocmd({ "CursorMoved" }, {
  callback = function()
    local mode = vim.api.nvim_get_mode().mode

    if mode ~= "v" or not static.preview_switch then
      return
    end

    codesnap.preview_code()
  end,
})

vim.api.nvim_create_autocmd({ "VimLeavePre" }, {
  pattern = "*",
  callback = function()
    client:stop()
  end,
})
