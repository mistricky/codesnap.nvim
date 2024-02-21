local table_utils = require("codesnap.utils.table")
local static = require("codesnap.static")
local client = require("codesnap.client")
local visual_utils = require("codesnap.utils.visual")

local main = {
  cwd = static.cwd,
  preview_switch = static.preview_switch,
}

function main.setup(config)
  static.config = table_utils.merge(static.config, config == nil and {} or config)

  if static.config.auto_load then
    client:start()
  end

  client:send("config_setup", static.config)
end

function main.preview_code()
  client:send("preview_code", { content = visual_utils.get_selected_text(), language = vim.bo.filetype })
end

function main.open_preview()
  client:send("open_preview")
end

function main.stop_client()
  client:stop()
end

return main
