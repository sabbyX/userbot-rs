# userbot-rs
Yet another userbot, but in rust!

## building
what? `cargo build`

## Architecture
...
#### `./`
`build.rs` - Contains code for extracting [grammers-client](https://github.com/lonami/grammers)'s version :shrug:
#### `./src/`
`tui.rs`, `cmd.rs` - Interactive login/configuration experience thingies
#### `./src/modules`
Contains all modules
#### `./src/modules/core/`
Contains update propagation/handler implementations
