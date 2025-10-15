local fetch = {}

local function get_os_arch()
  local uname = vim.loop.os_uname()
  local os_name = uname.sysname:lower()
  local arch = uname.machine:lower()

  -- Normalize OS names
  if os_name == "darwin" then
    os_name = "mac"
  elseif os_name == "windows_nt" then
    os_name = "windows"
  end

  -- Normalize architecture names
  if arch == "x86_64" or arch == "amd64" then
    arch = "x86_64"
  elseif arch == "aarch64" or arch == "arm64" then
    arch = "aarch64"
  end

  return os_name, arch
end

local function get_platform_lib_name()
  local os_name, arch = get_os_arch()
  local extension = "so"

  if os_name == "windows" then
    extension = "dll"
  elseif os_name == "mac" then
    extension = "dylib"
  end

  return string.format("%s-%s_generator.%s", os_name, arch, extension)
end

local function get_plugin_version()
  local path_utils = require("codesnap.utils.path")
  local sep = path_utils.get_separator()
  local project_toml_path =
    path_utils.join(sep, vim.fn.fnamemodify(debug.getinfo(1).source:sub(2), ":p:h:h:h"), "project.toml")

  local file = io.open(project_toml_path, "r")
  if not file then
    return nil
  end

  local content = file:read("*all")
  file:close()

  local version = content:match('version%s*=%s*"([^"]+)"')
  return version
end

local function download_lib(version, lib_name, dest_path)
  local url = string.format("https://github.com/mistricky/codesnap.nvim/releases/download/v%s/%s", version, lib_name)
  local platform_utils = require("codesnap.utils.platform")

  local cmd
  if platform_utils.is_windows() then
    -- Use PowerShell on Windows
    cmd = string.format(
      'powershell -Command "Invoke-WebRequest -Uri %s -OutFile %s"',
      vim.fn.shellescape(url),
      vim.fn.shellescape(dest_path)
    )
  else
    -- Use curl on Unix-like systems
    cmd = string.format("curl -L -o %s %s", vim.fn.shellescape(dest_path), vim.fn.shellescape(url))
  end

  local result = vim.fn.system(cmd)

  if vim.v.shell_error ~= 0 then
    error("Failed to download library: " .. result)
  end

  return true
end

function fetch.ensure_lib()
  local path_utils = require("codesnap.utils.path")
  local lib_name = get_platform_lib_name()
  local version = get_plugin_version()

  if not version then
    error("Could not determine plugin version")
  end

  -- Check if library already exists and is current version
  local sep = path_utils.get_separator()
  local lib_dir = path_utils.join(sep, vim.fn.fnamemodify(debug.getinfo(1).source:sub(2), ":p:h:h"), "libs")
  local lib_path = path_utils.join(sep, lib_dir, lib_name)
  local version_file = path_utils.join(sep, lib_dir, ".version")

  -- Create libs directory if it doesn't exist
  vim.fn.mkdir(lib_dir, "p")

  -- Check if we need to download/update
  local need_download = true
  if vim.fn.filereadable(lib_path) == 1 and vim.fn.filereadable(version_file) == 1 then
    local current_version = vim.fn.readfile(version_file)[1]
    if current_version == version then
      need_download = false
    end
  end

  if need_download then
    vim.notify("Downloading codesnap library for " .. lib_name .. "...")
    download_lib(version, lib_name, lib_path)
    vim.fn.writefile({ version }, version_file)
    vim.notify("Library downloaded successfully!")
  end

  return lib_path
end

return fetch
