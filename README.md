# CraftyMc
An open source alternative to curseforge, supporting modrinth for all your modpack consumption and creation needs.

In alpha stages. Heavily inspired by curseforge for ui purposes, for now.

Name may change.

## Technology

* [Tauri (Edge Webview)](https://tauri.studio/)
  - Rust
  - Vue3 & TS


## Preview


![app_1653166952](https://user-images.githubusercontent.com/4030546/169668926-1c878b8b-a49b-40f0-8b24-d6d48a85d473.gif)
* Preview is outdated and is from v0.2.0-alpha or earlier

### Current Roadmap, for beta release

 * Linux support for installing forge
 * Show externally installed mods in pack's modlist
 * Importing modpacks from modrinth
 * Add update checking and updating for mods
 * Support for fabric
 * Improve export dialog with save-as prompt
 * UI Improvements


#### Development

* `yarn` or `npm i` to install UI dependencies,
* `yarn serve` or `npm run serve` to run the web server for port 8080
* `yarn tauri dev` or `npm run dev` to run the application (requires web server to be running)
