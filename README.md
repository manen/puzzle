# `puzzle`

project puzzle is an attempt to make self-contained wasm applications and runners, interchangable with one-another, making it possible to develop an app/game once and run it anywhere puzzle has a runner

to make this possible, puzzle uses many modules to communicate between the app and the runner, all of which can be found in [`glue`](/glue)

## roadmap

- [x] `glue`: communication modules
- [x] `glue/log`, `glue/id`
- [ ] `glue/wgl/common`: wgl traits for backends
- [ ] `utils/gen/wgl_common`: codegen for `wgl_common`
- [ ] `glue/wgl/desktop`: desktop implementation for wgl
- [ ] `glue/wgl/wasm`: wasm bindings for wgl
- [ ] `utils/gen/wgl_wasm`: codegen for `wgl_wasm`
