local codesnap = require("codesnap")
local foo = require("foo")

foo.say_hello("Mist")

vim.api.nvim_create_user_command("CodeSnap", function() end, {})
