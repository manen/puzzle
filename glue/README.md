# `glue`

this directory contains the implementations and sdks to every module accessible from wasm

what we should aim for is a match in the wasm-facing and runtime-facing apis of these crates

## directory structure

- `glue/<name>/<name>`: reexport module, depending on feature flags
- `glue/<name>/wasm`: webassembly sdk
- `glue/<name>/runtime`: runtime implementation

## api

exposed from wasm:

- `puzzle_main() -> ()` - gets called on program start
- `puzzle_render() -> ()` - gets called on every redraw
