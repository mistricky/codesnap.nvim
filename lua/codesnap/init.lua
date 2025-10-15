local static = require("codesnap.static")
local table_utils = require("codesnap.utils.table")
local module = require("codesnap.module")
local config_module = require("codesnap.config")

local main = {
  cwd = static.cwd,
  highlight_mode_config = nil,
}

-- Prepare the path of the Rust module
-- Try to fetch pre-built library first, then fallback to development build
module.load_generator()

function main.setup(config)
  static.config = table_utils.merge_config(static.config, config == nil and {} or config)
end

-- Save snapshot to specified save_path
--- @param save_path string
function main.save(save_path)
  local generator = require("generator")

  if save_path == nil then
    error("Save path is not specified", 0)
  end

  generator.save(save_path, config_module.get_config())
  vim.cmd("delmarks <>")
  vim.notify("Save snapshot in " .. save_path .. " successfully!")
end

-- Copy snapshot into clipboard
function main.copy()
  local generator = require("generator")

  generator.copy(config_module.get_config())
  vim.cmd("delmarks <>")
  vim.notify("The snapshot is copied into clipboard successfully!")
end

-- Generate ASCII code snapshot and copy it into clipboard
function main.copy_ascii()
  local generator = require("generator")

  generator.copy_ascii(config_module.get_config())
  vim.cmd("delmarks <>")
  vim.notify("The ASCII code snapshot is copied into clipboard successfully!")
end

return main
