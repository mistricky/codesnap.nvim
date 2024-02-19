local path_utils = {}

function path_utils.back(path)
  local parsed_path, _ = path:gsub("/[^\\/]+/?$", "")

  return parsed_path
end

return path_utils
