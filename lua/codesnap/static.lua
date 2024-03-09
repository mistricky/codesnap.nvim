local path_utils = require("codesnap.utils.path")

-- pub mac_window_bar: bool,
--     pub watermark: Option<String>,
--     pub title: Option<String>,
--     pub code_font_family: String,
--     pub watermark_font_family: String,
--     pub code: String,
--     pub language: Option<String>,
--     pub extension: Option<String>,
--     pub save_path: Option<String>,

return {
  config = {
    mac_window_bar = true,
    watermark = "CodeSnap.nvim",
    title = "CodeSnap.nvim",
    code_font_family = "CaskaydiaCove Nerd Font",
    watermark_font_family = "Pacifico",
    save_path = "/Users/zhanhaozhao/repositories/codesnap.nvim/asd.png",
  },
  cwd = path_utils.back(path_utils.back(debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])"))),
  preview_switch = true,
}
