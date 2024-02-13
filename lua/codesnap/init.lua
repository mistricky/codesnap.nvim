local table_utils = require("utils.table")
local static = require("plugin-name.static")
local main = {}

function main.setup(config)
  static.config = table_utils.merge(static.config, config)
end

return main
