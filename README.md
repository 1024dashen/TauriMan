# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

为什么不能和PakePlus集成？
因为PakePLus本身包含了PP的ui内容，如果合在一起，会导致新的项目存在多余的PP内容，导致包体积过大，和性能下降。

## Recommended IDE Setup

-   [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# Add TauriApi

```shell
pnpm add -D @tauri-apps/cli@latest

"@tauri-apps/api": "^2",
"@tauri-apps/plugin-dialog": "^2.2.0",
"@tauri-apps/plugin-fs": "^2.2.0",
"@tauri-apps/plugin-http": "^2.2.0",
"@tauri-apps/plugin-opener": "^2",
"@tauri-apps/plugin-os": "^2.2.0",
"@tauri-apps/plugin-shell": "^2.2.0",
"@tauri-apps/plugin-store": "^2.2.0",
"@tauri-apps/plugin-updater": "^2.7.1",
"@tauri-apps/plugin-window-state": "^2.2.1",
```
