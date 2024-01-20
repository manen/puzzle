#!/usr/bin/bash

git clone git@github.com:manen/puzzle ~/puzzle
cd ~/puzzle
cargo build --manifest-path server/server/Cargo.toml --release
pkill server
run server
