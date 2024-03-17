![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/9fc40afc-cff8-4e18-a04f-e931d32a1fb7)



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



https://github.com/mistricky/codesnap.nvim/assets/22574136/99be72db-57d7-4839-91d0-2a9dfb1901ac



### Save the snapshot

Of course, you can use `CodeSnapSave` command to save the snapshot to path where you defined it in `config.save_path`
```lua
require("codesnap").setup({
  -- ...
  save_path = ...
})
```



https://github.com/mistricky/codesnap.nvim/assets/22574136/69b27e77-3dce-4bc3-8516-89ce636fe02d


## Breadcrumbs
Breadcrumbs are something to display the current snapshot file path, you can open it through config `has_breadcrumbs`:
```lua
require("codesnap").setup({
  -- ...
  has_breadcrumbs = true
})
```
The breadcrumbs look like:
![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/23274faa-36a9-4d41-88a5-e48c44b4d5bf)

### Custom path separator
The CodeSnap.nvim uses `/` as the separator of the file path by default, of course, you can specify any symbol you prefer as the custom separator:
```lua
require("codesnap").setup({
  -- ...
  has_breadcrumbs = true
  breadcrumbs_separator = "👉"
})
```

![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/84b80d0f-1467-4bdf-9cbd-aede868f93aa)



## Custom background
The `CodeSnap.nvim` comes with many beautiful backgrounds preset, you can set any background you like by setting `bg_theme` to its name, just like:
```lua
require("codesnap").setup({
  -- The "default" background is one you see at the beginning of the README
  bg_theme = "default"
})
```
<table>
  <tr>
    <th>bamboo</th>
    <th>sea</th>
  </tr>
  <tr>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/ce3a387b-61a5-42ba-8f71-1b4949f5e148" width="650" />
    </td>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/122790b6-6365-402c-806a-dfc78dabbc06" width="650" />
    </td>
  </tr>

  <tr>
    <th>peach</th>
    <th>grape</th>
  </tr>
  <tr>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/c0ec9dc1-cd8b-463e-9f2d-ab2e1e3a9831" width="650" />
    </td>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/b573786b-70ed-4006-89c7-20bed115c9cc" width="650" />
    </td>
  </tr>


  <tr>
    <th>dusk</th>
        <th>summer</th>
  </tr>
  <tr>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/e3bb5222-542d-4c32-b78b-8cf4695feec9" width="650" />
    </td>
    <td>
      <img src="https://github.com/mistricky/codesnap.nvim/assets/22574136/98ced31a-091b-4ed8-9bd6-bb5b502a7db2" width="650" />
    </td>
  </tr>
</table>

### Solid color background
If you prefer solid color background, you can set `bg_color` to your preferred color. For example:
```lua
require("codesnap").setup({
  -- ...
  bg_color = "#535c68"
})
```

![CodeSnap](https://github.com/mistricky/codesnap.nvim/assets/22574136/a600c2e4-4c60-4ec0-b2fc-3b41481048dc)



## Watermark
Watermark is something that makes screenshots more personalized, but if you don't like watermark just set it as an empty string to hide it.
```lua
require("codesnap").setup({
  -- ...
  watermark = ""
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
    bg_theme = "default"
}
```

## License
MIT.
