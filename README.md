# Mdscript Language Server

The beginnings of a Language Server + VSCode Extension for Mdscript.

## Functionality

This Language Server works for plain text file. It currently has 2 usable features:
 - auto-complete the word "cheese"
 - hover will show "Hello World!" on every word

## Structure

```
.
├── client // Language Client
│   ├── src
│   │   ├── test // End to End tests for Language Client / Server (probably doesn't work right anymore)
│   │   └── extension.ts // Language Client entry point
├── package.json // The extension manifest.
└── rust-server // Language Server written in Rust
    ├── src
    │   └── main.rs // Language Server entry point
    └── target
        └── debug // Build folder
            └── mdls_server.exe // Executable for Language Server

```

## Running the Sample

- Run `npm install` in this folder. This installs all necessary npm modules in both the client and server folder
- Open VS Code on this folder.
- Press Ctrl+Shift+B to compile the client and server.
- Switch to the Debug viewlet.
- Select `Launch Client` from the drop down.
- Run the launch config.
<!-- - If you want to debug the server as well use the launch configuration `Attach to Server`
- In the [Extension Development Host] instance of VSCode, open a document in 'plain text' language mode.
  - Type `j` or `t` to see `Javascript` and `TypeScript` completion.
  - Enter text content such as `AAA aaa BBB`. The extension will emit diagnostics for all words in all-uppercase. -->
