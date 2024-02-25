![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/c8982b8c-c273-4257-9fef-f0f9134fd9a1)
<p align="center">

<img src="https://img.shields.io/badge/Neovim-57A143?logo=neovim&logoColor=fff&style=for-the-badge" alt="Neovim" />

<img src="https://img.shields.io/badge/Made%20With%20Lua-2C2D72?logo=lua&logoColor=fff&style=for-the-badge" alt="made with lua" >

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/release.yml?style=for-the-badge&label=release" alt="release action status" />

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/lint.yml?style=for-the-badge&label=Lint" alt="release action status" />

</p>

<h1 align="center">CodeSnap.nvim</h1>
<p align="center">📸 Snapshot plugin that can make pretty code snapshots with real-time previews for Neovim</p>

> [!NOTE]
> This plugin is currently in its early stages and may have some bugs, please feel free to submit issues and PRs.

## ✨Features
- 🔥 Real-time preview
- 🤩 Beautiful code snap template
- 😎 Custom watermark and window style
- 💻 Mac style title bar
- 👏 [WIP] Custom template background
- 🤖 [WIP] Generate snapshot just one command
  

## Prerequirements
- Rust environment required for compiling codesnap.nvim plugin server source code, visit [Install Rust](https://www.rust-lang.org/tools/install) for more detail.

## Install
```lua
{ "mistricky/codesnap.nvim", build = "make" },
```

## Usage 
For take a screenshot, the `codesnap.nvim` provides a command named `CodeSnapPreviewOn` to open the preview page, and then you can switch to visual mode and select code you want, and finally just click the copy button on the preview page, that's all :)

https://github.com/mistricky/codesnap.nvim/assets/22574136/5e1a023e-142f-49e8-b24f-707da3728fd5

## Commands
```shell
CodeSnapPreviewOn # Open preview page

-- The following commands are planned but not implemented yet. (welcome PR :))
CodeSnap # Take a code snap and copy it into the clipboard
```

## Configuration
Define your custom config using `setup` function
```lua
require("codesnap").setup({...})
```

There is a default config:
```lua
{
    mac_window_bar = true,-- (Optional) MacOS style title bar switch
    opacity = true, -- (Optional) The code snap has some opacity by default, set it to false for 100% opacity 
    watermark = "CodeSnap.nvim", -- (Optional) you can custom your own watermark, but if you don't like it, just set it to ""
    preview_title = "CodeSnap.nvim", -- (Optional) preview page title
    editor_font_family = "CaskaydiaCove Nerd Font", -- (Optional) preview code font family
    watermark_font_family = "Pacifico", -- (Optional) watermark font family
}
```

## License
MIT.
