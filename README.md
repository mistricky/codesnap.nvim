![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/9fc40afc-cff8-4e18-a04f-e931d32a1fb7)



<p align="center">

<img src="https://img.shields.io/badge/For Neovim 0.9+-57A143?logo=neovim&logoColor=fff&style=for-the-badge" alt="Neovim" />

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/release.yml?style=for-the-badge&label=release" alt="release action status" />

<img src="https://img.shields.io/github/actions/workflow/status/mistricky/codesnap.nvim/lint.yml?style=for-the-badge&label=Lint" alt="release action status" />

<a href="https://github.com/mistricky/codesnap.nvim/issues">
	<img alt="Issues" src="https://img.shields.io/github/issues/mistricky/codesnap.nvim?style=for-the-badge&logo=github&color=%23ffbd5e">
</a>
<a href="https://github.com/mistricky/codesnap.nvim/blob/main/LICENSE">
	<img alt="License" src="https://img.shields.io/github/license/mistricky/codesnap.nvim?style=for-the-badge&logo=github&color=%235ef1ff">
</a>
<a href="https://github.com/mistricky/codesnap.nvim/stars">
	<img alt="stars" src="https://img.shields.io/github/stars/mistricky/codesnap.nvim?style=for-the-badge&logo=github&color=%23bd5eff">
</a>

<img src="https://img.shields.io/badge/Made%20With%20Lua-2C2D72?logo=lua&logoColor=fff&style=for-the-badge" alt="made with lua" >

<img src="https://img.shields.io/badge/Written%20in%20Rust-B7410E?logo=rust&logoColor=fff&style=for-the-badge" alt="written in rust" >

<a href="https://dotfyle.com/plugins/mistricky/codesnap.nvim">
	<img src="https://dotfyle.com/plugins/mistricky/codesnap.nvim/shield?style=for-the-badge" />
</a>

</p>

<h1 align="center">CodeSnap.nvim</h1>
<p align="center">📸 Snapshot plugin with rich features that can make pretty code snapshots for Neovim</p>

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

- [🚣Migration](#migration)
- [✨Features](#features)
- [Prerequirements](#prerequirements)
- [Install](#install)
  - [Compile from source](#compile-from-source)
  - [Compile on ARM](#compile-on-arm)
  - [Keymappings](#keymappings)
- [Usage](#usage)
  - [Copy into the clipboard](#copy-into-the-clipboard)
    - [Copy into clipboard on Linux Wayland](#copy-into-clipboard-on-linux-wayland)
  - [Save the snapshot](#save-the-snapshot)
  - [Highlight code block](#highlight-code-block)
    - [How to use](#how-to-use)
  - [Specify language extension](#specify-language-extension)
- [Breadcrumbs](#breadcrumbs)
  - [Show workspace in breadcrumbs](#show-workspace-in-breadcrumbs)
  - [Custom path separator](#custom-path-separator)
- [Line number](#line-number)
- [Custom background](#custom-background)
  - [Solid color background](#solid-color-background)
  - [Customize background padding](#customize-background-padding)
- [Watermark](#watermark)
- [Commands](#commands)
- [Configuration](#configuration)
- [Contribution](#contribution)
  - [Contributors](#contributors)
- [License](#license)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## 🚣Migration
If you have installed v0.x before, this chapter will show you what break changes version v1.x introduced.

- The `CodeSnapPreviewOn` command is not supported, if you prefer live-preview, you can pin `CodeSnap.nvim` version to `v0.0.11` to continue using this command.
- The `opacity` and `preview_title` config has been removed from v1.0.0
- The `editor_font_family` was renamed to `code_font_family`

v1.x has a different architecture and better performance than v0.x, and v1.x can generate screenshots directly without an open browser. We recommend you upgrade to v1.x for a better experience.

## ✨Features
- 🤩 Beautiful code snap template
- 😎 Custom watermark and window style
- 💻 Beautiful Mac-style title bar
- 🤖 Generate snapshots using only a single command
- 🍞 Breadcrumbs for display file path
- 🌊 More beautiful background theme
- 🔢 Support for display line number make sharing code snapshot easier

## Prerequirements
- Neovim 0.9.0+

## Install
We recommend using [Lazy.nvim](https://github.com/folke/lazy.nvim) to install CodeSnap.nvim, but you can still use another plugin manager you prefer.

**Lazy.nvim**
```lua
{ "mistricky/codesnap.nvim", build = "make" },
```
**Packer**
```lua
use {'mistricky/codesnap.nvim', run = 'make'}
```

**Vim-Plug**
```lua
Plug 'mistricky/codesnap.nvim', { 'do': 'make' }
```

It's worth mentioning that the screenshot feature is implemented by a module called `generator` written in Rust, when `make` the project, the CodeSnap.nvim will mount a precompiled cross-compile `generator` shared file into the plugin that depends on what OS you are using. 

We precompiled the following targets:
- x86_64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin

If your platform is in the above list, you can just run `make` after the plugin is installed like the above examples do, CodeSnap.nvim will automatically mount the shared file into the plugin. This means you don't need any Rust utils to compile manually from source.

### Compile from source
You need to install Rust development environment before compiling from source, you can refer [Install Rust](https://www.rust-lang.org/tools/install) for more detail.

Please keep in mind, cross-compile to these platforms only helps a portion of users to have out-of-box experience, if your platform is not in the above targets list, you still need to compile from source using `make build_generator`, for instance using Lazy:

```lua
{ "mistricky/codesnap.nvim", build = "make build_generator" },
```

We always recommend you to compile CodeSnap.nvim from `source` instead of using the precompiled shared file, because the correctness and consistency of compiling from source are always higher than cross-compiling.

### Compile on ARM
If you try to compile CodeSnap.nvim on ARM architecture, you may need to install additional dependencies to compile it, thanks @matteocavestri mentioned in https://github.com/mistricky/codesnap.nvim/issues/53#issuecomment-2032088162

```shell
export CC=gcc
sudo dnf install libuv libuv-devel # On RHEL based systems
sudo apt-get install libtool libuv1-dev # On Debian based systems
```

### Keymappings
If you use `Lazy.nvim` as your package manager, here are some examples show you how to configure keymappings for CodeSnap:
```lua
{
  "mistricky/codesnap.nvim",
  build = "make build_generator",
  keys = {
    { "<leader>cc", "<cmd>CodeSnap<cr>", mode = "x", desc = "Save selected code snapshot into clipboard" },
    { "<leader>cs", "<cmd>CodeSnapSave<cr>", mode = "x", desc = "Save selected code snapshot in ~/Pictures" },
  },
  opts = {
    save_path = "~/Pictures",
    has_breadcrumbs = true,
    bg_theme = "bamboo",
  },
}
```

## Usage 
`CodeSnap.nvim` provides the following two ways to take snapshots of currently selected code

### Copy into the clipboard
To take a beautiful snapshot use CodeSnap.nvim, you can just use `CodeSnap` command to generate a snapshot of the current selected code, then the `CodeSnap.nvim` will write the snapshot into the clipboard, and you can paste it anywhere you want.



https://github.com/mistricky/codesnap.nvim/assets/22574136/99be72db-57d7-4839-91d0-2a9dfb1901ac

#### Copy into clipboard on Linux Wayland
Copy screenshots directly into the clipboard is cool, however, it doesn't work well on wl-clipboard, because the wl-clipboard can't paste the content which come from exited processes. As Hyprland document say:


> When we copy something on Wayland (using wl-clipboard) and close the application we copied from, the copied data disappears from the clipboard and we cannot paste it anymore. So to fix this problem we can use a program called as wl-clip-persist which will preserve the data in the clipboard after the application is closed. 


If you using CodeSnap.nvim on wl-clipboard, you can refer [wl-clip-persist](https://github.com/Linus789/wl-clip-persist), it reads all the clipboard data into memory and then overwrites the clipboard with the data from our memory to persist copied data.

### Save the snapshot

Of course, you can use `CodeSnapSave` command to save the snapshot to path where you defined it in `config.save_path`
```lua
require("codesnap").setup({
  -- The save_path must be ends with .png, unless when you specified a directory path,
  -- CodeSnap will append an auto-generated filename to the specified directory path
  -- For example:
  -- save_path = "~/Pictures"
  -- parsed: "~/Pictures/CodeSnap_y-m-d_at_h:m:s.png"
  -- save_path = "~/Pictures/foo.png"
  -- parsed: "~/Pictures/foo.png"
  save_path = ...
})
```

https://github.com/mistricky/codesnap.nvim/assets/22574136/69b27e77-3dce-4bc3-8516-89ce636fe02d

### Highlight code block

CodeSnap allows you to take code snapshots with highlights code blocks, we provide two commands for this scenario:

```shell
CodeSnapHighlight # Take code snapshot with highlights code blocks and copy it into the clipboard
CodeSnapSaveHighlight # Take code snapshot with highlights code blocks and save it somewhere
```

#### How to use
For take a code snapshot with highlights code blocks and save it somewhere. First you need to select code which you want to snapshot, then enter the command `CodeSnapSaveHighlight` to open a window show you the selected code which from previous step, now you can select code which you want to highlight, finally press the Enter key, CodeSnap will generate a snapshot with highlight blocks and save it in save_path.

Here is an example video:

https://github.com/mistricky/codesnap.nvim/assets/22574136/bea0bf6c-8fc9-4d09-9cab-4e1e6f47899c




### Specify language extension
In some scenarios, CodeSnap.nvim cannot auto-detect what language syntax should used to highlight code, for example, shell script can have no extension, they specify interpreters using shebang.

CodeSnap.nvim won't read the whole content of the file, thus cannot detect what language syntax is this, in this case, you can specify extension explicitly, for instance:

```
CodeSnapSave sh
CodeSnap sh
```


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

### Show workspace in breadcrumbs
Breadcrumbs hide the workspace name by default, if you want to display workspace in breadcrumbs, you can just set `show_workspace` as true.
```lua
require("codesnap").setup({
  -- ...
  has_breadcrumbs = true
  show_workspace = true
})
```

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


## Line number
We also support displaying line number, you can set `has_line_number` to true to display line number.
```lua
require("codesnap").setup({
  // ...
	has_line_number = true,
})
```

![image](https://github.com/mistricky/codesnap.nvim/assets/22574136/3a5999b1-bb2a-4646-8d69-609be1d28140)


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

### Customize background padding
CodeSnap allows you to customize the padding of background using `bg_x_padding`, `bg_y_padding` and `bg_padding`, the default value is:
```lua
require("codesnap").setup({
  bg_x_padding = 122,
  bg_y_padding = 82,
  bg_padding = null
})
```

If you want to hide background, you can set `bg_padding` to `0` in your config:
```lua
require("codesnap").setup({
  -- ...
  bg_padding = 0
})
```

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
**Lua**
```lua
local codesnap <const> = require("codesnap")

-- Take a snapshot of the currently selected code and copy the snapshot into the clipboard
codesnap.copy_into_clipboard()

-- Save the snapshot of the currently selected code and save it on the disk
codesnap.save_snapshot()
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
    bg_theme = "default",
    breadcrumbs_separator = "/",
    has_breadcrumbs = false,
    has_line_number = false,
    show_workspace = false,
    min_width = 0,
    bg_x_padding = 122,
    bg_y_padding = 82,
}
```

## Contribution
CodeSnap.nvim is a project that will be maintained for the long term, and we always accepts new contributors, please feel free to submit PR & issues.

The commit message convention of this project is following [commitlint-wizardoc](https://github.com/wizardoc/commitlint-wizardoc).

### Contributors
Thanks to all contributors for their contributions and works they have done.

<img src="CONTRIBUTORS.svg" />

## License
MIT.
