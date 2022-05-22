# mc-mod-manager

Extremely alpha stages. Heavily inspired by curseforge for ui purposes, for now.

Name is still pending.

Built with Tauri (rust + edge webview) with Vue.js & TS on the UI

## Preview

![app_1653166952](https://user-images.githubusercontent.com/4030546/169668926-1c878b8b-a49b-40f0-8b24-d6d48a85d473.gif)

### Current Roadmap, for alpha release

1. Overall UI mockup (mostly done)
2. UI functionality (somewhat done)
3. Backend rust code
   2. Installing modloader solution
   3. Installing, deleting, updating, editing mods
    * Filtering and some minor bug squashing


### Using

First setup requires two manual setups:
a. Minecraft launcher must be installed in Documents\McModManager\Instance, 
    Copying from your actual minecraft install (C:\program files (X86)\ does work fine)
b. Modloaders (like forge) need to be manually installed, then setup in instance's manifest

#### Development

`yarn serve` to run the web server for port 8080

`tauri dev` to run the actual application
