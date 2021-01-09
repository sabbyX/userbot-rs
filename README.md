# userbot-rs
Yet another userbot, but in rust!

## building
🤷‍♀️

for debug build,
```commandline
cargo build
```
for release build (**recommended**),
```commandline
cargo build --release
```

Binary can found in  `targets/debug/userbot_rs.exe` or `targets/release/userbot_rs.exe`

## Running

**step 1**: Install binary, You can install stable binary from [releases](https://github.com/sabbyX/userbot-rs/releases). Or nightly builds from commit artifacts section!

**step 2**: Running

Once binary is installed as `userbot_rs.exe` or similar
```commandline
userbot_rs.exe --api-id 00000 --api-hash abcdefghi0124
```
> Tip: Once you give `API ID` or `API_HASH`, it get stored and reused, ie, no need to pass `API ID/HASH` again after first run

You can see more command line help by
```commandline
userbot_rs.exe --help
```

## Bug Reporting

Just file a issue [here](https://github.com/sabbyX/userbot-rs/issues)

## Architecture
This repository contains both [userbot-rs](crates/userbot-rs) and [userbot-rs-macros](crates/userbot-rs-macros).

---
### userbot-rs
Ready to use userbot binary (you can get latest binary from commit artifacts section).

&nbsp; &nbsp; **directories**

&nbsp; &nbsp; &nbsp; &nbsp; - `src/` contains startup things such as login system, module initializer and command line things

&nbsp; &nbsp; &nbsp; &nbsp; - `src/config/` contains configuration system

&nbsp; &nbsp; &nbsp; &nbsp; - `src/module/core/` contains _update dispatcher_ and supporting utils for [userbot-rs-macros](crates/userbot-rs-macros).

&nbsp; &nbsp; &nbsp; &nbsp; - `src/module/mod.rs:function:intialize` where all modules get registered to dispatcher

&nbsp; &nbsp; &nbsp; &nbsp; - `src/module/` contains all modules.

### userbot-rs-macros
proc-macro library to enhance handler system. Heavily inspired from [Carapax Project](https://github.com/tg-rs/carapax)

```rust
#[handler(command = "cmd")]
fn my_handler(/* parameters here */) { /* do something */ }

// expands above into something like this

struct MyHandler;

impl Handler for MyHandler {
    fn handler(/* parameters here */ ) { /* do something */ }
    fn command_policy() { /* the function holds handler's command policy */ }
}

```

&nbsp; &nbsp; **directories**

&nbsp; &nbsp; &nbsp; &nbsp; - `src/lib.rs` splits the expansions into two, _command_ and _handler_

&nbsp; &nbsp; &nbsp; &nbsp; - `src/command.rs` handle the command input, parses them into [CommandPolicy](crates/userbot-rs/src/modules/core/command.rs) enum

&nbsp; &nbsp; &nbsp; &nbsp; - `src/handler.rs` expands handler into struct implementing [Handler](crates/userbot-rs/src/modules/core/handler.rs) trait 
