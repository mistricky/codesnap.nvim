local path_module = {}

local function expand_path(path)
  if type(path) ~= "string" then
    return path
  end
  if path:sub(1, 1) == "~" then
    local home = os.getenv("HOME") or os.getenv("USERPROFILE") or ""
    return home .. path:sub(2)
  end
  return path
end

function path_module.expand_paths_in_config(config)
  if config.themes_folders and #config.themes_folders > 0 then
    config.themes_folders = vim.tbl_map(expand_path, config.themes_folders)
  end

  if config.fonts_folders and #config.fonts_folders > 0 then
    config.fonts_folders = vim.tbl_map(expand_path, config.fonts_folders)
  end
  return config
end

return path_module
