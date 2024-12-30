local visual_utils = require("codesnap.utils.visual")
local path_utils = require("codesnap.utils.path")
local string_utils = require("codesnap.utils.string")
local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local config_module = {}

local assets_folder = static.cwd .. "/assets"

-- Auto generated codesnap filename based on date_pattern
local function auto_generate_snap_filename(date_pattern)
  return os.date(date_pattern)
end

-- If the save_path is already configured, but no explicit filename is specified,
-- it will be replaced with auto-generated filename
local function parse_save_path(save_path, date_pattern)
  if save_path == nil or string_utils.ends_with(save_path, "png") then
    return save_path
  end

  local parsed_save_path = string_utils.ends_with(save_path, "/") and save_path or save_path .. "/"

  return parsed_save_path .. auto_generate_snap_filename(date_pattern)
end

local function get_file_path(show_workspace)
  local relative_path = path_utils.get_relative_path()

  return show_workspace and path_utils.get_workspace() .. "/" .. relative_path or relative_path
end

function config_module.get_config(extension)
  local code = visual_utils.get_selected_text()
  local start_line_number = visual_utils.get_start_line_number()

  if string_utils.is_str_empty(code) then
    error("No code is selected", 0)
    return
  end

  local config = table_utils.merge({
    code = code,
    extension = extension,
    code_file_path = vim.fn.expand("%:p"),
    fonts_folder = assets_folder .. "/fonts",
    themes_folder = assets_folder .. "/themes",
    theme = "base16-onedark",
    file_path = static.config.has_breadcrumbs and get_file_path(static.config.show_workspace) or "",
    start_line_number = static.config.has_line_number and start_line_number or nil,
  }, static.config)

  config.save_path = parse_save_path(config.save_path, config.filename_date_pattern)

  return config
end

return config_module
