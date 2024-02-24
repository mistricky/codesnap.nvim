local codesnap = require("codesnap")
-- local client = require("codesnap.client")

-- vim.api.nvim_create_user_command("CodeSnap", function()
--   client:send("copy")
-- end, {})

vim.api.nvim_create_user_command("CodeSnapPreviewOn", function()
  codesnap.open_preview()
end, {})

vim.api.nvim_create_user_command("CodeSnapPreviewOff", function()
  codesnap.stop_client()
end, {})

local validModes = {
  ["v"] = true,
  ["V"] = true,
}
vim.api.nvim_create_autocmd({ "CursorMoved", "ModeChanged" }, {
  callback = function()
    local mode = vim.api.nvim_get_mode().mode

    if not validModes[mode] or not codesnap.preview_switch then
      return
    end

    codesnap.preview_code()
  end,
})

vim.api.nvim_create_autocmd({
  "VimLeavePre",
  "BufDelete",
  "InsertEnter",
}, {
  pattern = "*",
  callback = function()
    codesnap.stop_client()
  end,
})
