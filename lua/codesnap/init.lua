local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local string_utils = require("codesnap.utils.string")
local config_module = require("codesnap.config")

local main = {
  cwd = static.cwd,
  preview_switch = static.preview_switch,
}

function main.setup(config)
  static.config = table_utils.merge(static.config, config == nil and {} or config)
end

function main.copy_into_clipboard(extension)
  require("generator").copy_into_clipboard(config_module.get_config(extension))
  vim.cmd("delmarks <>")
  vim.notify("Save snapshot into clipboard successfully")
end

function main.save_snapshot(extension)
  if string_utils.is_str_empty(static.config.save_path) then
    error(
      "If you want to save snapshot in somewhere, you should config the save_path before, refer: https://github.com/mistricky/codesnap.nvim?tab=readme-ov-file#save-the-snapshot",
      0
    )
  end

  local matched_extension = string.match(static.config.save_path, "%.(.+)$")

  if matched_extension ~= "png" and matched_extension ~= nil then
    error("The extension of save_path should be .png", 0)
  end

  local config = config_module.get_config(extension)

  require("generator").save_snapshot(config)
  vim.cmd("delmarks <>")
  ---@diagnostic disable-next-line: need-check-nil
  vim.notify("Save snapshot in " .. config.save_path .. " successfully")
end

return main
