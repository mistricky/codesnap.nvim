local codesnap = require("codesnap")

vim.api.nvim_create_user_command("CodeSnap", function()
  codesnap.copy_into_clipboard()
end, { nargs = "*", range = "%" })

vim.api.nvim_create_user_command("CodeSnapSave", function()
  codesnap.save_snapshot()
end, { nargs = "*", range = "%" })
