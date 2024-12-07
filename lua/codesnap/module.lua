local module = {}

local path_utils = require("codesnap.utils.path")

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
  return path_utils.join("/", RUST_BUILD_DIR, filename .. "." .. module.get_lib_extension())
end

return module
