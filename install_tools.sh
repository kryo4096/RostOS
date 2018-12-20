#!/bin/bash

cargo install --path rost_fs/fscreate
cargo install bootimage
cargo install cargo-xbuild

rustup toolchain install nightly
rustup override add nightly
rustup component add rust-src



