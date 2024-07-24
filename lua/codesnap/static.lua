local path_utils = require("codesnap.utils.path")
-- Get user os
-- If linux, use XDG_PICTURE_DIR, if mac use ~/Pictures, if windows use FOLDERID_Pictures (If support is added back)
local default_save_path = nil
local os_name = vim.loop.os_uname().sysname
if os_name == "Linux" then
  default_save_path = os.getenv("XDG_PICTURES_DIR") or (os.getenv("HOME") .. "/Pictures")
elseif os_name == "Darwin" then
  default_save_path = os.getenv("HOME") .. "/Pictures"
else
  error("codesnap.nvim only supports Linux and MacOS")
end

return {
  config = {
    mac_window_bar = true,
    title = "CodeSnap.nvim",
    code_font_family = "CaskaydiaCove Nerd Font",
    watermark_font_family = "Pacifico",
    watermark = "CodeSnap.nvim",
    bg_theme = "default",
    breadcrumbs_separator = "/",
    has_breadcrumbs = false,
    has_line_number = false,
    show_workspace = false,
    min_width = 0,
    bg_x_padding = 122,
    bg_y_padding = 82,
    save_path = default_save_path,
  },
  cwd = path_utils.back(path_utils.back(debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])"))),
  preview_switch = true,
}
