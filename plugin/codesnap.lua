local codesnap = require("codesnap")

vim.api.nvim_create_user_command("CodeSnap", function()
  codesnap.setup()
end, {})
