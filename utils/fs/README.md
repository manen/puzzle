# `utils/fs`

the main crate is [`fs_trait`](trait/README.md)

the point is you can create any virtual filesystem of your choosing that won't actually exist, only in code and only on demand, when called for with a `read_dir` or an `open`.

files in this filesystem aren't carved in stone and rigid like in regular filesystems, rather any type can be a "file" that implements `io::Read` and `io::Write`. that means you can use your file as a regular tcp socket, or maybe even some programmatic function that accepts encoded binary as input. or just json. be lazy

these filesystems can then be linked together between computers using [`fs_socketio_server`](socketio_server/README.md) and [`fs_socketio_client`](socketio_client/README.md), paving the way for some (ab)usecases, you could use a virtual filesystem as an api and such

(a wasmtime [wasi](https://wasi.dev/) bind generator is on the roadmap, with a couple of lines you could have an app compiled to wasi and read files as usual, not knowing that those files aren't real and all that data travelled through the wasi interface and a socketio connection)
