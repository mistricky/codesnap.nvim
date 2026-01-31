<img width="1819" height="1788" alt="cover" src="https://github.com/user-attachments/assets/25547efa-e1ad-4fb5-a8fa-b5dad8217de1" />


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

<img src="https://img.shields.io/badge/Written%20in%20Rust-DEA584?logo=rust&logoColor=fff&style=for-the-badge" alt="written in rust" >

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
  - [Windows Support](#windows-support)
- [Usage](#usage)
  - [Copy into the clipboard](#copy-into-the-clipboard)
    - [Copy into clipboard on Linux Wayland](#copy-into-clipboard-on-linux-wayland)
  - [Save the snapshot](#save-the-snapshot)
  - [Highlight code block](#highlight-code-block)
    - [How to use](#how-to-use)
  - [Take ASCII snapshot](#take-ascii-snapshot)
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

### Upgrade from v0.x to v1

If you have installed v0.x before, this chapter will show you what break changes version v1.x introduced.

- The `CodeSnapPreviewOn` command is not supported, if you prefer live-preview, you can pin `CodeSnap.nvim` version to `v0.0.11` to continue using this command.
- The `opacity` and `preview_title` config has been removed from v1.0.0
- The `editor_font_family` was renamed to `code_font_family`

v1.x has a different architecture and better performance than v0.x, and v1.x can generate screenshots directly without an open browser. We recommend you upgrade to v1.x for a better experience.

### Upgrade from v1 to v2

CodeSnap.nvim v2 bring a lot of new features and improvements, the most important change is that we rewrote the screenshot generator using [CodeSnap](https://github.com/codesnap-rs/codesnap), this library makes CodeSnap.nvim faster and more stable than before.

And there is no need to setup Rust environment to compile CodeSnap.nvim anymore, we precompiled the `generator` shared file for common platforms:

- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- x86_64-apple-darwin
- aarch64-apple-darwin
- windows-x86_64-msvc

For most cases, you can use CodeSnap.nvim out-of-box without any additional setup. 🍵

Configuration is **completely different** between v1 and v2, which is following [config.rs](https://github.com/codesnap-rs/codesnap/blob/main/core/src/config.rs) in CodeSnap. Which means you can use same configuration in both CodeSnap CLI and CodeSnap.nvim. Click [here]() see what 
current config looks like.

#### Windows Support
We are excited to announce that CodeSnap.nvim now supports Windows! 🎉, but we don't have enough time to test it on Windows, so if you find any issues on Windows, please let us know by creating an issue.

## Prerequirements
- Neovim 0.9.0+

## Installation
We recommend using [Lazy.nvim](https://github.com/folke/lazy.nvim) to install CodeSnap.nvim, but you can still use another plugin manager you prefer.

**Lazy.nvim**
```lua
{ "mistricky/codesnap.nvim", tag = "v2.0.0" }
```

> Maybe you are CodeSnap.nvim v1 user, you may notice that we remove the `build` option in v2, because we don't need to compile the Rust code anymore, we precompiled the `generator` shared file for common platforms, you can find the precompiled files in [releases](https://github.com/mistricky/codesnap.nvim/releases) page. So when you first install v2, CodeSnap.nvim will download the precompiled file automatically, it may take a few seconds to download the file, please be patient.


## Keymappings
TODO

## ✨Features

### Consume Snapshot
There are two ways to consume the snapshot you took using CodeSnap.nvim:

#### Copy into clipboard
Copy the snapshot directly into clipboard, then you can paste it anywhere you want.

Run `CodeSnap` command, CodeSnap.nvim will generate a snapshot of the currently selected code and write it into clipboard.

```
CodeSnap
```

#### Copy into clipboard on Linux Wayland
Copy screenshots directly into the clipboard is cool, however, it doesn't work well on wl-clipboard, because the wl-clipboard can't paste the content which come from exited processes. As Hyprland document say:


> When we copy something on Wayland (using wl-clipboard) and close the application we copied from, the copied data disappears from the clipboard and we cannot paste it anymore. So to fix this problem we can use a program called as wl-clip-persist which will preserve the data in the clipboard after the application is closed. 


If you using CodeSnap.nvim on wl-clipboard, you can refer [wl-clip-persist](https://github.com/Linus789/wl-clip-persist), it reads all the clipboard data into memory and then overwrites the clipboard with the data from our memory to persist copied data.

#### Save the snapshot
Save the snapshot into a file, you can specify the path where you want to save it

Run `CodeSnapSave` command, CodeSnap.nvim will generate a snapshot of the currently selected code and save it in the path you specified in config.

CodeSnap.nvim supports saving snapshot in `PNG`, `SVG` and `HTML` format, you can specify the file extension in the path you provided, for example:

```shell
CodeSnapSave /path/to/your/snapshot.png
CodeSnapSave /path/to/your/snapshot.svg
CodeSnapSave /path/to/your/snapshot.html
```

### ASCII snapshot

CodeSnap.nvim also supports taking ASCII art snapshot, you can use `CodeSnapASCII` command to take a snapshot in ASCII format and copy it into clipboard.

This feature is not useful for most cases, but it's FUN and lightweight, if you want to paste your code in somewhere like comment,  ASCII snapshot may be a good choice.

```lua
╭────────────────────────────────────────────────────────────────╮
│ codesnap.nvim/lua/codesnap/config.lua                          │
│────────────────────────────────────────────────────────────────│
│ 24 local code_content = {                                      │
│ 25   content = code,                                           │
│ 26   start_line_number = start_line_number,                    │
│ 27   file_path = get_file_path(static.config.show_workspace),  │
│ 28 }                                                           │
╰────────────────────────────────────────────────────────────────╯
```

As you can see, the ASCII snapshot is just a plain text, without any background, line number, watermark, etc. But it still has key information like code content, line number and file path, which can be useful if someone want to know where the code is from.

Really hope you like this feature! 🤗


### Highlight code block

CodeSnap allows you to take code snapshots with highlights code blocks, we provide two commands for this scenario:

```shell
CodeSnapHighlight # Take code snapshot with highlights code blocks and copy it into the clipboard
CodeSnapSaveHighlight # Take code snapshot with highlights code blocks and save it somewhere
```

#### How to use
For take a code snapshot with highlights code blocks and save it somewhere. First you need to select code which you want to snapshot, then enter the command `CodeSnapSaveHighlight` to open a window show you the selected code which from previous step, now you can select code which you want to highlight _(if any - you can use these without actually highlighting anything)_, finally press the Enter key, CodeSnap will generate a snapshot with highlight blocks and save it in save_path.

![Highlight Demo](/doc/highlight_demo.png)



### Breadcrumbs
Breadcrumbs are really useful tool to display the current snapshot file path, you can enable it by setting `breadcrumbs.enable` to true:

```lua
require("codesnap").setup({
	-- ...
	snapshot_config = {
    -- ...
		code_config = {
			breadcrumbs = {
				enable = true,
				separator = "/",
				color = "#80848b",
				font_family = "CaskaydiaCove Nerd Font",
      }
		}
	}
})
```

Once you enable breadcrumbs, CodeSnap will display the current file path on the top of the snapshot, like this:
![Breadcrumbs Demo](/doc/breadcrumbs_demo.png)


### Line number
Line number is another useful tool to display the current line number of the code, you can enable it by setting `show_line_number` to true:

```lua
require("codesnap").setup({
  -- ...
  show_line_number = true
})
```

![Line Number Demo](/doc/line_number_demo.png)


### Watermark
You can set your own watermark by setting `watermark.content` to your own watermark content.

```lua
require("codesnap").setup({
  -- ...
  watermark = {
    content = "CodeSnap.nvim",
    font_family = "Pacifico",
    color = "#ffffff",
  }
})
```

![Watermark Demo](/doc/watermark_demo.png)

### Custom theme
For CodeSnap.nvim, theme is primarily defined by two parts:
- The background theme
- The code theme

Custom background theme is easy to understand, which is defined by `Background` enum:

```rust
pub enum Background {
    Solid(String),
    Gradient(LinearGradient),
}
```

As you can see, there have two types of background theme:
- Solid(String): A solid color background
- Gradient(LinearGradient): A gradient background

If you prefer solid background, you can just leave it as a solid color string, for example:

```lua
background = "#FFFFFF"
```

Above code will generate a solid white background.

![Cutom background white](/doc/custom_background_solid.png)

CodeSnap.nvim use gradient background by default, you can specify the gradient colors by setting `background.stops` to a table of colors, for example:

```lua
"background": {
    "start": {
      "x": 0,
      "y": 0
    },
    "end": {
      "x": "max",
      "y": "max"
    },
    "stops": [
      {
        "position": 0,
        "color": "#EBECB2"
      },
      {
        "position": 0.28,
        "color": "#F3B0F7"
      },
      {
        "position": 0.73,
        "color": "#92B5F0"
      },
      {
        "position": 0.94,
        "color": "#AEF0F8"
      }
    ]
}
```

![Custom background gradient](/doc/custom_background_gradient.png)

Or you prefer transparent background, for example:
```lua
background = "#00000000",
```

![Transparent background](/doc/transparent_demo.png)

#### Custom code theme

For code theme, it's a little bit complex than background theme, you may notice that there only have one config to specify the code theme, which is `theme`, which is a string that represents the code theme name.

```lua
snapshot_config = {
  theme = "candy",
}
```

Above code will use the "candy" theme as the code theme.

"candy" is a built-in theme name, so you can just use it directly.

CodeSnap.nvim use [syntect](https://github.com/trishume/syntect) as code theme engine, which supports Sublime Text theme format, if you want to use your own theme, follow the steps below:

1. Specify `themes_folders` to the path of your own theme files, for example:
```lua
snapshot_config = {
  themes_folders = {
    "~/.config/codesnap/themes",
  },
}
```

2. Put your own theme files in the path you specified, for example:
```
~/.config/codesnap/themes/my_theme.tmTheme
```

3. Use your own theme by setting `theme` to the name of your own theme, for example:
```lua
theme = "my_theme",
```

That's all, now you can use your own theme to take code snapshots.

But you may notice that it's not so easy to use theme you want to use, and if you are VSCode user before, you may know that VSCode has a lot of themes, and you can just search and install the theme you want to use.

Fortunately, CodeSnap.nvim has a built-in theme parser which can convert VSCode theme format to Sublime Text theme format, for example, we want to use the "One Hunter" theme from VSCode (which also is the demo theme in README), we just need to few steps to use it:

1. Construct a "Asset URL", which is a URL that points to the theme file, for example:
```shell
# The prefix "vercel@" is the theme name, you can use any name you want, but it must be provided and unique.
vercel@https://raw.githubusercontent.com/Railly/one-hunter-vscode/refs/heads/main/themes/OneHunter-Vercel-color-theme.json
```

2. Use the "Asset URL" to set `theme` in config, for example:
```lua
theme = "vercel@https://raw.githubusercontent.com/Railly/one-hunter-vscode/refs/heads/main/themes/OneHunter-Vercel-color-theme.json",
```

That's all, now you can use the "One Hunter" theme to take code snapshots.

###  More beautiful themes
The benefit of using "Asset URL" is that you can easily share and store your snapshot config without refer any external resources.

CodeSnap.nvim offers greater flexibility for you to craft your own snapshot theme, you can share your own theme on [Awesome CodeSnap](https://github.com/codesnap-rs/awesome-codesnap?tab=readme-ov-file)

We are looking forward to your amazing snapshot theme! 🤗

## Commands
```shell
CodeSnap # Take a snapshot of the currently selected code and copy the snapshot into the clipboard

CodeSnapSave <path> # Save the snapshot of the currently selected code and save it on the disk

CodeSnapASCII # Take a code snapshot in ASCII format

CodeSnapHighlight # Take code snapshot with highlights code blocks and copy it into the clipboard
```
**Lua**
```lua
local codesnap <const> = require("codesnap")

-- Take a snapshot of the currently selected code and copy the snapshot into the clipboard
codesnap.copy()

-- Save the snapshot of the currently selected code and save it on the disk
codesnap.save(path)
```

## Configuration
Define your custom config using `setup` function
```lua
require("codesnap").setup({...})
```

There is a default config:
```lua
{
  show_line_number = true,
  highlight_color = "#ffffff20",
  show_workspace = true,
  snapshot_config = {
    theme = "candy",
    window = {
      mac_window_bar = true,
      shadow = {
        radius = 20,
        color = "#00000040",
      },
      margin = {
        x = 82,
        y = 82,
      },
      border = {
        width = 1,
        color = "#ffffff30",
      },
      title_config = {
        color = "#ffffff",
        font_family = "Pacifico",
      },
    },
    themes_folders = {},
    fonts_folders = {},
    line_number_color = "#495162",
    command_output_config = {
      prompt = "❯",
      font_family = "CaskaydiaCove Nerd Font",
      prompt_color = "#F78FB3",
      command_color = "#98C379",
      string_arg_color = "#ff0000",
    },
    code_config = {
      font_family = "CaskaydiaCove Nerd Font",
      breadcrumbs = {
        enable = true,
        separator = "/",
        color = "#80848b",
        font_family = "CaskaydiaCove Nerd Font",
      },
    },
    watermark = {
      content = "CodeSnap.nvim",
      font_family = "Pacifico",
      color = "#ffffff",
    },
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
}
```

Actually, these config are come from the [CodeSnap](https://github.com/codesnap-rs/codesnap) library, you can refer to the [CodeSnap](https://github.com/codesnap-rs/codesnap) documentation to learn more about the configuration.


## Contribution
CodeSnap.nvim is a project that will be maintained for the long term, and we always accepts new contributors, please feel free to submit PR & issues.

The commit message convention of this project is following [commitlint-wizardoc](https://github.com/wizardoc/commitlint-wizardoc).

### Contributors
Thanks to all contributors for their contributions and works they have done.

<img src="CONTRIBUTORS.svg" />

## License
MIT.
