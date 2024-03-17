local string_utils = require("codesnap.utils.string")
local path_utils = {}

function path_utils.back(path)
  local parsed_path, _ = path:gsub("/[^\\/]+/?$", "")

  return parsed_path
end

function path_utils.get_relative_path()
  local full_file_path = vim.fn.expand("%:p")
  local cwd = vim.fn.getcwd()

  return full_file_path:gsub(string_utils.escape(cwd), ""):sub(2)
end

return path_utils
