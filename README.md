# mc-mod-manager

Extremely alpha stages. Heavily inspired by curseforge for ui purposes, for now.

Name is still pending.

Built with Tauri (rust + edge webview) with Vue.js & TS on the UI

## Preview

![app_1653166952](https://user-images.githubusercontent.com/4030546/169668926-1c878b8b-a49b-40f0-8b24-d6d48a85d473.gif)

### Current Roadmap, for alpha release

1. Overall UI mockup (mostly done)
2. Backend rust code
   1. Testing modloader install solution
   2. Filter mod lists
   3. Testing of entire fresh install

#### Development

* `yarn` or `npm i` to install UI dependencies,
* `yarn serve` or `npm run serve` to run the web server for port 8080
* `yarn tauri dev` or `npm run dev` to run the application (requires web server to be running)
