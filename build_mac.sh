#!/bin/bash

# Make the final output directory
if [ ! -e "bin" ]; then
    mkdir bin
fi
pushd bin
if [ -e "macosx" ]; then
    rm -rf ./macosx
fi
mkdir macosx
cd macosx
mkdir x64
popd

Rust64="nightly-x86_64-apple-darwin"

rustup update $Rust64

##################################
# Build serialization library

pushd lib/serialization
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
popd
cp lib/serialization/target/release/libserialization.* bin/macosx/x64

##################################
# Build imageload library

pushd lib/imageload
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
popd
cp lib/imageload/target/release/libimageload.* bin/macosx/x64

##################################
# Build messageipc library

pushd lib/messageipc
rustup run $Rust64 cargo clean
rustup run $Rust64 cargo build --release
popd
cp lib/messageipc/target/release/libmessageipc.* bin/macosx/x64

