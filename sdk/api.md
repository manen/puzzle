# API

exposed from application:

- `puzzle_main() -> ()` - gets called on program start
- `puzzle_render()` -> () - gets called on every redraw

exposed to application:

- `puzzle_log(level: u32, ptr: *const u8, len: u32) -> ()` (level is log::Level as u32)
- `puzzle_log_flush() -> ()`

- `jigsaw_start() -> i32` (-1 if error)
- `jigsaw_width() -> i32` (-1 if jigsaw hasn't been started)
- `jigsaw_height() -> i32` (-1 if jigsaw hasn't been started)
- `jigsaw_debug_text(x: u32, y: u32, ptr: *const u8, len: u32) -> bool` (-1 if error)
