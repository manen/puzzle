# `id`

this module handles identification between the wasm application and the runner, performing a sort of handshake

exposed to wasm:

- `puzzle_id_api_version_major() -> u32`
- `puzzle_id_api_version_minor() -> u32`
- `puzzle_id_api_version_patch() -> u32`

exposed from wasm:

- `puzzle_id_name_len() -> u32`
- `puzzle_id_name() -> *const u8`
- `puzzle_id_version_major() -> u32`
- `puzzle_id_version_minor() -> u32`
- `puzzle_id_version_path() -> u32`
- `puzzle_id_target_api_version_major() -> u32`
- `puzzle_id_target_api_version_minor() -> u32`
- `puzzle_id_target_api_version_patch() -> u32`

these functions should only be called by the runner after `puzzle_main` has run to let the app initialize
