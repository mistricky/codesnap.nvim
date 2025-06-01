local visual_utils = require("codesnap.utils.visual")
local path_utils = require("codesnap.utils.path")
local string_utils = require("codesnap.utils.string")
local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local config_module = {}

local function get_file_path(show_workspace)
  local relative_path = path_utils.get_relative_path()

  return show_workspace and path_utils.get_workspace() .. "/" .. relative_path or relative_path
end

function config_module.get_config()
  local code = visual_utils.get_selected_text()
  local start_line_number = visual_utils.get_start_line_number()

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
    -- start_line_number = static.config.has_line_number and start_line_number or nil,
  })

  -- config.save_path = parse_save_path(config.save_path)
  return config
end

return config_module
