local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local generator = require("generator")
local string_utils = require("codesnap.utils.string")
local visual_utils = require("codesnap.utils.visual")

local function get_extension()
  local file_path = vim.fn.expand("%:p") -- 获取当前文件的完整路径
  local file_extension = string.match(file_path, "%.([^%.]+)$") -- 提取文件后缀名
  return file_extension
end

local main = {
  cwd = static.cwd,
  preview_switch = static.preview_switch,
}

function main.setup(config)
  static.config = table_utils.merge(static.config, config == nil and {} or config)
end

local function get_config()
  local code = visual_utils.get_selected_text()
  local extension = get_extension()

  if string_utils.is_str_empty(code) then
    error("Please select code which you want to take snapshot first")
  end

  if string_utils.is_str_empty(extension) then
    error("Cannot detect current filetype")
  end

  return table_utils.merge({
    code = code,
    extension = extension,
  }, static.config)
end

function main.copy_into_clipboard()
  generator.copy_into_clipboard(get_config())
  vim.cmd("delmarks <>")
  vim.notify("Save snapshot into clipboard successfully")
end

function main.save_snapshot()
  if string_utils.is_str_empty(static.config.save_path) then
    error("Cannot find save_path from config")
  end

  generator.save_snapshot(get_config())
  vim.cmd("delmarks <>")
  vim.notify("Save snapshot in " .. static.config.save_path .. " successfully")
end

return main
