local path_utils = require("codesnap.utils.path")

return {
  config = {
    window = {
      mac_window_bar = true,
      shadow = 20,
      margin = {
        x = 82,
        y = 82,
      },
      border = {
        color = "#ffffff30",
      },
    },
    code = {
      font_family = "CaskaydiaCove Nerd Font",
      theme = "candy",
    },
    watermark = {
      content = "CodeSnap",
      font_family = "Pacifico",
      color = "#ffffff",
    },
    scale_factor = 3,
    background = {
      start = {
        x = 0,
        y = 0,
      },
      ["end"] = {
        x = "max",
        y = 0,
      },
      stops = {
        {
          position = 0,
          color = "#6bcba5",
        },
        {
          position = 1,
          color = "#caf4c2",
        },
      },
    },
  },
  cwd = path_utils.back(path_utils.back(debug.getinfo(1, "S").source:sub(2):match("(.*[/\\])"))),
}
