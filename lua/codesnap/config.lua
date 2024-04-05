local visual_utils = require("codesnap.utils.visual")
local path_utils = require("codesnap.utils.path")
local string_utils = require("codesnap.utils.string")
local static = require("codesnap.static")
local list_utils = require("codesnap.utils.list")
local table_utils = require("codesnap.utils.table")
local config_module = {}

local assets_folder = static.cwd .. "/assets"

-- Get extension of cureent file
local function get_file_info()
  local file_path = vim.fn.expand("%:p")
  local filename, extension = string.match(file_path, "([^\\/%.]+)%.?([^\\/%.]*)$")

  return string_utils.convert_empty_to_nil(filename), string_utils.convert_empty_to_nil(extension)
end

-- Some files have no extension, but they still need to highlighting correctly,
-- in this case, use filename instead of extension to highlighting code
-- e.g. Dockerfile, Pipefile
local function parse_file_extension_by_highlighting_file_presets(filename, extension)
  local lowercase_filename = string.lower(filename)

  return extension or lowercase_filename
end

local function parse_extension(specify_extension)
  local filename, file_extension = get_file_info()

  return specify_extension
    or file_extension
    or parse_file_extension_by_highlighting_file_presets(filename, file_extension)
end

function config_module.get_config(specify_extension)
  local code = visual_utils.get_selected_text()
  local extension = specify_extension or parse_extension(specify_extension)

  if string_utils.is_str_empty(code) then
    error("No code is selected", 0)
    return
  end

  if string_utils.is_str_empty(extension) then
    error("Cannot detect current filetype", 0)
  end

  return table_utils.merge({
    code = code,
    extension = extension,
    fonts_folder = assets_folder .. "/fonts",
    themes_folder = assets_folder .. "/themes",
    theme = "base16-onedark",
    file_path = static.config.has_breadcrumbs and path_utils.get_relative_path() or "",
  }, static.config)
end

return config_module
