local module = {}

local path_utils = require("codesnap.utils.path")
local fetch = require("codesnap.fetch")
local platform = require("codesnap.utils.platform")

local OS_LIB_EXTENSION_MAP = {
  mac = "dylib",
  osx = "dylib",
  windows = "dll",
  linux = "so",
}

local sep = path_utils.get_separator()
local RUST_BUILD_DIR = path_utils.with_dir_name(
  ".." .. sep .. ".." .. sep .. ".." .. sep .. "generator" .. sep .. "target" .. sep .. "debug"
)

function module.get_lib_extension()
  local extension = OS_LIB_EXTENSION_MAP[jit.os:lower()]

  return extension or "so"
end

-- Get the path of the the generator file
function module.generator_file_path(is_debug)
  if is_debug then
    local filename = platform.is_windows() and "generator" or "libgenerator"

    return path_utils.join(sep, RUST_BUILD_DIR, filename .. "." .. module.get_lib_extension())
  end

  -- First try to use pre-built library from libs directory
  local ok, lib_path = pcall(fetch.ensure_lib)

  if ok and lib_path and vim.fn.filereadable(lib_path) == 1 then
    -- Add the libs directory to package.cpath
    local libs_dir = path_utils.join(sep, vim.fn.fnamemodify(debug.getinfo(1).source:sub(2), ":p:h:h"), "libs")
    package.cpath = path_utils.join(";", package.cpath, path_utils.join(sep, libs_dir, "?"))
    return lib_path
  end

  error("Failed to load the generator library. Please ensure it is built correctly.")
end

function module.load_generator(is_debug)
  local generator_path = module.generator_file_path(is_debug)

  package.cpath = path_utils.join(";", package.cpath, generator_path)

  if module.generator == nil then
    module.generator = require("generator")
  end

  return module.generator
end

return module
