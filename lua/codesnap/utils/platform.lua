local platform_utils = {}

local current_os_name = vim.loop.os_uname().sysname

function platform_utils.match_os(matches_table)
  local fn = matches_table[current_os_name]

  if fn == nil then
    error("codesnap.nvim not supported on " .. current_os_name)
  end

  return fn()
end

return platform_utils
