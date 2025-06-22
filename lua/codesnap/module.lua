local module = {}

local path_utils = require("codesnap.utils.path")
local fetch = require("codesnap.fetch")

local OS_LIB_EXTENSION_MAP = {
  mac = "dylib",
  osx = "dylib", 
  windows = "dll",
  linux = "so",
}

local RUST_BUILD_DIR = path_utils.with_dir_name("../../../generator/target/debug")

function module.get_lib_extension()
  local extension = OS_LIB_EXTENSION_MAP[jit.os:lower()]

  return extension or "so"
end

function module.generator_file(filename)
  -- First try to use pre-built library from libs directory
  local ok, lib_path = pcall(fetch.ensure_lib)
  if ok and lib_path and vim.fn.filereadable(lib_path) == 1 then
    -- Add the libs directory to package.cpath
    local libs_dir = path_utils.join("/", vim.fn.fnamemodify(debug.getinfo(1).source:sub(2), ":p:h:h"), "libs")
    package.cpath = path_utils.join(";", package.cpath, path_utils.join("/", libs_dir, "?"))
    return lib_path
  end
  
  -- Fallback to development build
  return path_utils.join("/", RUST_BUILD_DIR, filename .. "." .. module.get_lib_extension())
end

return module
