# API

exposed from application:

- `puzzle_main() -> ()` - gets called on program start
- `puzzle_render()` -> () - gets called on every redraw

exposed to application:

- `puzzle_log(level: u32, ptr: *const u8, len: u32) -> ()` (level is log::Level as u32)
- `puzzle_log_flush() -> ()`
