local path_utils = require("codesnap.utils.path")

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
  },
  cwd = path_utils.back(path_utils.back(debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])"))),
  preview_switch = true,
}
