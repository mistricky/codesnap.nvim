local visual_utils = require("codesnap.utils.visual")
local path_utils = require("codesnap.utils.path")
local string_utils = require("codesnap.utils.string")
local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local module = require("codesnap.module")
local config_module = {}

local function get_file_path(show_workspace)
  local relative_path = path_utils.get_relative_path()

  return show_workspace and path_utils.get_workspace() .. "/" .. relative_path or relative_path
end

function config_module.get_config()
  local code = visual_utils.get_selected_text()
  local start_line_number = static.config.show_line_number and visual_utils.get_start_line_number() or nil

  if string_utils.is_str_empty(code) then
    error("No code is selected", 0)
    return
  end

  local code_content = {
    content = code,
    start_line_number = start_line_number,
    file_path = get_file_path(static.config.show_workspace),
  }

  local config = table_utils.assign(static.config.snapshot_config, {
    content = code_content,
    theme = module.load_generator(true).parse_code_theme(static.config.snapshot_config.theme),
  })

  return config
end

return config_module
