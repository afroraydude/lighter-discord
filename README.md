# lighter-discord

A lightweight Discord client for Windows, Linux and macOS.

## Why?

The reason for creating lighter-discord is because of the slowness of Discord's underlying electron engine.
Instead, lighter-discord uses a system-integrated webview engine, which can decrease RAM usage significantly.

Notifications still work, voice and video also work as well. Extra features
such as Push-to-talk hotkeys and custom themes/plugins are in the works.

## Features

-   **Lightweight** - lighter-discord is a lightweight Discord client that uses less resources than the official client.
-   **Cross-platform** - lighter-discord is available for Windows, Linux and macOS.

## Installation

### macOS

Download the latest version of lighter-discord from the [releases page](https://github.com/afroraydude/lighter-discord/releases) and place the .app file in your Applications folder.

### Windows

Download the latest version of lighter-discord from the [releases page](https://github.com/afroraydude/lighter-discord/releases) and extract the contents of the zip file to a folder of your choice.

### Linux

View the [Linux installation guide](https://github.com/afroraydude/lighter-discord/blob/master/docs/linux-install.md) for instructions on how to install lighter-discord on Linux.

## Building from source

1. Clone the repository
2. Make sure you have the required dependencies installed ([see here](https://tauri.app/v1/guides/getting-started/prerequisites/) for more information)
3. Install the Tauri CLI: `cargo install tauri-cli` (if you already have the Tauri CLI installed, make sure it's up to date)
4. Open a terminal in the repository directory and run `cargo tauri build`
5. All done! The binaries will be in the `target/release` directory
