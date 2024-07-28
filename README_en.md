<h1 align="center">
  <img src="./client/supercv-vue/src-tauri/icons/Square310x310Logo.png" alt="SuperCV" width="228" />
  <br>
  A Clipboard Enhancement Tool📋
  <br>
  Better Suited for CV Engineers
  <br>
</h1>

[简体中文](README.md)

## Introduce

SuperCV is built with Tauri and Rust, serving as an enhanced clipboard tool.

It supports clipboard history and search functions for text, images, and files, and enables clipboard synchronization across multiple devices on a local network.
Common questions refer to [FAQ](./docs/faq.md)

## Preview

|                Text                |              Image               |               File                |
| :--------------------------------: | :------------------------------: | :-------------------------------: |
| ![text](./docs/imgs/show_text.png) | ![img](./docs/imgs/show_img.png) | ![img](./docs/imgs/show_file.png) |

**<u>Use `CommandOrControl+Shift+L` to summon the SuperCV interface</u>**

## Installation

Please go to the release page to download the corresponding installation package: [Release Page](https://github.com/Zeke-chin/SuperCV/releases)

Confirmed support for desktop platforms Windows(x64), Linux(x64), MacOS (Intel/Apple)

Other desktop platforms not tested

## Features

- **Lightweight** - Packaged with tauri, providing excellent performance with zero-overhead rust
- Supports local network clipboard sharing between multiple devices (to be implemented)
- Clipboard history and search for text, image, and file types🔍
- Separate retention time settings for text, images, and files
- Supports setting the number of preview items

## TODO

- 🗒️ Shortcut key configuration
- 🗒️ Support for multiple files
- 🗒️ Server-side synchronization feature
- ....

## Development Guide

1. Install dependencies
   - Rust: [install](https://www.rust-lang.org/tools/install), version >= `1.63`
   - Node: [install](https://nodejs.org/en/download/package-manager), version >= `20.15`
   - (If you are on Linux): `sudo apt-get update && sudo apt-get install -y libgtk-3-dev webkit2gtk-4.0 libappindicator3-dev librsvg2-dev patchelf`
2. Clone the repository

   - `git clone https://github.com/Zeke-chin/SuperCV`

   - `cd  SuperCV`

3. Install frontend dependencies
   - `npm install` or `yarn`
4. Start development
   - `npm run tauri dev`

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
</head>
<body>
    <h2>Developers</h2>
    <ul class="developer-list">
        <li class="developer-item">
            <img src="https://avatars.githubusercontent.com/u/84116651?v=4" alt="img" width="75" height="75">
            <a href="https://github.com/zeke-chin" class="name">zeke-chin</a>
        </li>
        <li class="developer-item">
            <img src="https://avatars.githubusercontent.com/u/71913459?v=4" alt="img" width="75" height="75">
            <a href="https://github.com/langchou" class="name">langchou</a>
        </li>
        <li class="developer-item">
            <img src="https://avatars.githubusercontent.com/u/74230079?v=4" alt="img" width="75" height="75">
            <a href="https://github.com/N1body" class="name">N1body</a>
        </li>
    </ul>
</body>
</html>

## Acknowledgments

- [ChurchTao/clipboard-rs](https://github.com/ChurchTao/clipboard-rs): Cross-platform clipboard API (text | image | rich text | html | files | monitoring changes)
- [tauri-apps/tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [clash-verge-rev/clash-verge-rev](https://github.com/clash-verge-rev/clash-verge-rev): Continuation of Clash Verge - A Clash Meta GUI based on Tauri (Windows, MacOS, Linux)
