# `glue`

this directory contains the implementations and sdks to every module accessible from wasm

what we should aim for is a match in the wasm-facing and x86_64-facing apis of these crates

## directory structure

- `glue/<name>/<name>`: reexport module, depending on target platform
- `glue/<name>/wasm`: webassembly sdk
- `glue/<name>/runtime`: runtime implementation

## API

exposed from application:
todo: remove puzzle_main

- `puzzle_main() -> ()` - gets called on program start
- `puzzle_render() -> ()` - gets called on every redraw

exposed to application:

- `puzzle_log(level: u32, ptr: *const u8, len: u32) -> ()` (level is log::Level as u32)
- `puzzle_log_flush() -> ()`
