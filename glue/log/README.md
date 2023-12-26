# `log`

usage (from wasm):

```rs
puzzle_log::init().unwrap();
log::info!("hello world!");
```

exposed to wasm:

- `puzzle_log(level: u32, ptr: *const u8, len: u32) -> ()` (level is log::Level as u32)
- `puzzle_log_flush() -> ()`
