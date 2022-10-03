# CraftyMc
An open source alternative to curseforge, supporting modrinth for all your modpack consumption and creation needs.

In alpha stages. Heavily inspired by curseforge for ui purposes, for now.

Supports creating custom modpacks, importing and exporting modrinth modpacks

## Update

A lot of stuff broke right when I released it, don't have the motivation anymore to work on this

## Technology

* [Tauri (Edge Webview)](https://tauri.studio/)
  - Rust
  - Vue3 & TS


## Preview

![app_1653166952](https://user-images.githubusercontent.com/4030546/169668926-1c878b8b-a49b-40f0-8b24-d6d48a85d473.gif)
* Preview is outdated and is from v0.2.0-alpha or earlier

### Current Roadmap

See the [project page](https://github.com/Jackzmc/craftymc/projects/1) for up to date progress.

- Full Linux support
- Download mods on import (from normal zip)
- Fabric Support
- Mod Update Checking & Installing
- UI Improvements
- Code Cleanup
- Full Testing

#### Development

* `yarn` or `npm i` to install UI dependencies,
* `yarn serve` or `npm run serve` to run the web server for port 8080
* `yarn tauri dev` or `npm run dev` to run the application (requires web server to be running)
