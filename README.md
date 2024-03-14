![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/6e54fea2-4d88-45df-9a7b-1d90a4bb0ea7)

<p align="center">

<img src="https://img.shields.io/badge/Neovim-57A143?logo=neovim&logoColor=fff&style=for-the-badge" alt="Neovim" />

<img src="https://img.shields.io/badge/Made%20With%20Lua-2C2D72?logo=lua&logoColor=fff&style=for-the-badge" alt="made with lua" >

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/release.yml?style=for-the-badge&label=release" alt="release action status" />

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/lint.yml?style=for-the-badge&label=Lint" alt="release action status" />

</p>

<h1 align="center">CodeSnap.nvim</h1>
<p align="center">📸 Snapshot plugin that can make pretty code snapshots with real-time previews for Neovim</p>

## ✨Features
- 🔥 Real-time preview
- 🤩 Beautiful code snap template
- 😎 Custom watermark and window style
- 💻 Mac style title bar
- 🤖 Generate snapshot just one command
- 👏 [WIP] Custom template background
- 🔢 [WIP] Column number
- 🍞 [WIP] Breadcrumbs
  

## Prerequirements
- Neovim 9.0+

## Install
Recommend using [Lazy.nvim](https://github.com/folke/lazy.nvim) for installation, but you can still use other plugin manager you prefer to install it.

**Lazy.nvim**
```lua
{ "mistricky/codesnap.nvim", build = "make"},
```

## Usage 
`CodeSnap.nvim` provide the following two ways to take snapshot of current selected code

### Copy into clipboard

To take a beautiful snapshot use CodeSnap.nvim, you can just use `CodeSnap` command to generate a snapshot of the current selected code, then the `CodeSnap.nvim` will write the snapshot into clipboard, and you can paste it anywhere you want.

### Save the snapshot

Of course, you can use `CodeSnapSave` command to save the snapshot to path where you defined it in `config.save_path`
```lua
require("codesnap").setup({
  -- ...
  save_path: ...
})
```

## Commands
```shell
CodeSnap # Take snapshot of current selected code and copy the snapshot into clipboard

CodeSnapSave # Save the snapshot of current selected code and save it on disk
```

## Configuration
Define your custom config using `setup` function
```lua
require("codesnap").setup({...})
```

There is a default config:
```lua
{
    mac_window_bar = true,
    watermark = "CodeSnap.nvim",
    title = "CodeSnap.nvim",
    code_font_family = "CaskaydiaCove Nerd Font",
    watermark_font_family = "Pacifico",
}
```

## License
MIT.
