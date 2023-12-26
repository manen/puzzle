# `id`

this module handles identification between the wasm application and the runner, performing a sort of handshake

usage from wasm:

```rs
id::ensure_api_version()?;
let runner: Runner = id::runner();
```

usage from wasmtime backend:

```rs
// wasmtime
id::ensure_app_api_version(&mut store, &instance)?;
let app: App = id::app(&mut store, &instance)?;
```

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
