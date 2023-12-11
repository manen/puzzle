# API

exposed from application:

- `puzzle_main() -> ()` - gets called on program start

exposed to application:

- `print(ptr: *const c_char) -> ()`
- `eprint(ptr: *const c_char) -> ()`
- `trace(ptr: *const c_char) -> ()`
