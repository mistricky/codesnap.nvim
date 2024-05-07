local string_utils = require("codesnap.utils.string")
local path_utils = {}

function path_utils.get_escaped_cwd()
  local cwd = vim.fn.getcwd()

  return string_utils.escape(cwd)
end

function path_utils.back(path)
  local parsed_path, _ = path:gsub("/[^\\/]+/?$", "")

  return parsed_path
end

function path_utils.get_workspace()
  local cwd = vim.fn.getcwd()
  local _, _, workspace = string.find(cwd, "/([^/]+)$")

  return workspace == nil and "" or workspace
end

function path_utils.get_relative_path()
  local full_file_path = vim.fn.expand("%:p")

  return full_file_path:gsub(path_utils.get_escaped_cwd(), ""):sub(2)
end

return path_utils
