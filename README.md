# `puzzle`

project puzzle is an attempt to make self-contained wasm applications and runners, interchangable with one-another, making it possible to develop an app/game once and run it anywhere puzzle has a runner

to make this possible, puzzle uses the [webassembly system interface](https://wasi.dev/) and some other interfaces ([`glue`](/glue)) to communicate between the app and the runner
