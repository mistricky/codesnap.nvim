local string_utils = require("codesnap.utils.string")
local platform_utils = require("codesnap.utils.platform")
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

-- Get default save path by OS
-- If Linux, use XDG_PICTURE_DIR
-- if mac use ~/Pictures
-- if windows use FOLDERID_Pictures (If support is added back)
function path_utils.get_default_save_path()
  local home_picture_folder = os.getenv("HOME") .. "/Pictures"

  return platform_utils.match_os({
    Darwin = function()
      return home_picture_folder
    end,
    Linux = function()
      return os.getenv("XDG_PICTURES_DIR") or home_picture_folder
    end,
  })
end

return path_utils
