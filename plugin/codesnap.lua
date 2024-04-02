local codesnap = require("codesnap")

local function take_snapshot(take_snapshot_function)
  return function(detail)
    local args = detail.fargs

    take_snapshot_function(args[1])
  end
end

vim.api.nvim_create_user_command("CodeSnap", take_snapshot(codesnap.copy_into_clipboard), { nargs = "*", range = "%" })

vim.api.nvim_create_user_command("CodeSnapSave", take_snapshot(codesnap.save_snapshot), { nargs = "*", range = "%" })
