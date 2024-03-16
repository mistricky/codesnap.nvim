![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/4afe1462-9325-40e2-856e-be8a7ce4fdad)



<p align="center">

<img src="https://img.shields.io/badge/Neovim-57A143?logo=neovim&logoColor=fff&style=for-the-badge" alt="Neovim" />

<img src="https://img.shields.io/badge/Made%20With%20Lua-2C2D72?logo=lua&logoColor=fff&style=for-the-badge" alt="made with lua" >

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/release.yml?style=for-the-badge&label=release" alt="release action status" />

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/lint.yml?style=for-the-badge&label=Lint" alt="release action status" />

</p>

<h1 align="center">CodeSnap.nvim</h1>
<p align="center">📸 Snapshot plugin that can make pretty code snapshots with real-time previews for Neovim</p>

> [!WARNING]  
> **v1.0.0** will bring some break changes
> - The `CodeSnapPreviewOn` command is not supported, if you prefer live-preview, you can pin `CodeSnap.nvim` version to `v0.0.11` to continue using this command.
> - The `opacity` and `preview_title` config has been removed from v1.0.0
> - The `editor_font_family` was renamed to `code_font_family`

## ✨Features
- 🤩 Beautiful code snap template
- 😎 Custom watermark and window style
- 💻 Beautiful Mac-style title bar
- 🤖 Generate snapshots using only a single command
- 👏 [WIP] Custom template background
- 🔢 [WIP] Column number
- 🍞 [WIP] Breadcrumbs

## Prerequirements
- Neovim 9.0+

## Install
Recommend using [Lazy.nvim](https://github.com/folke/lazy.nvim) for installation, but you can still use another plugin manager you prefer.

**Lazy.nvim**
```lua
{ "mistricky/codesnap.nvim", build = "make", version = "^1" },
```

### Build manually
Since v0.0.1 was released, the CodeSnap.nvim will cross-compile for the following three targets, then the CodeSnap.nvim will automatically determine which package to use based on your system, you no longer need to have Rust environment if everything goes smoothly.
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin

If CodeSnap.nvim on your system still not works fine, there are a lot of reasons depending on your system arch or OS version, well you can try to build CodeSnap manually using the following config:
```lua
{ "mistricky/codesnap.nvim", build = "make build_generator", version = "^1" },
```

## Usage 
`CodeSnap.nvim` provides the following two ways to take snapshots of currently selected code

### Copy into the clipboard
To take a beautiful snapshot use CodeSnap.nvim, you can just use `CodeSnap` command to generate a snapshot of the current selected code, then the `CodeSnap.nvim` will write the snapshot into the clipboard, and you can paste it anywhere you want.

https://github.com/mistricky/codesnap.nvim/assets/22574136/88d9fe9e-d938-4d82-a1e4-b7170ca47dd9


### Save the snapshot

Of course, you can use `CodeSnapSave` command to save the snapshot to path where you defined it in `config.save_path`
```lua
require("codesnap").setup({
  -- ...
  save_path: ...
})
```


https://github.com/mistricky/codesnap.nvim/assets/22574136/7b156a89-1e0d-48cd-b062-af5a460973ba


## Watermark
Watermark is something that makes screenshots more personalized, but if you don't like watermark just set it as empty string to hide it.
```lua
require("codesnap").setup({
  -- ...
  watermark: ""
})
```

## Commands
```shell
CodeSnap # Take a snapshot of the currently selected code and copy the snapshot into the clipboard

CodeSnapSave # Save the snapshot of the currently selected code and save it on the disk
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
    title = "CodeSnap.nvim",
    code_font_family = "CaskaydiaCove Nerd Font",
    watermark_font_family = "Pacifico",
    watermark = "CodeSnap.nvim",
}
```

## License
MIT.
