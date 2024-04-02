local codesnap = require("codesnap")

-- The func param is a function that come from rust side, the function
-- may raise exception to user side, the run_generator_function is used to
-- handle these function and print it friendly
local function run_generator_function(func)
  xpcall(func, function(err)
    print(err)
  end)
end

local function take_snapshot(take_snapshot_function)
  return function(detail)
    local args = detail.fargs

    run_generator_function(function()
      take_snapshot_function(args[1])
    end)
  end
end

vim.api.nvim_create_user_command("CodeSnap", take_snapshot(codesnap.copy_into_clipboard), { nargs = "*", range = "%" })

vim.api.nvim_create_user_command("CodeSnapSave", take_snapshot(codesnap.save_snapshot), { nargs = "*", range = "%" })
