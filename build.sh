#!/bin/bash

# mkdir ./out/

cargo build --release --verbose --target=x86_64-pc-windows-gnu
cargo build --release --verbose --target=x86_64-unknown-linux-gnu

cargo build --manifest-path ./wasm/Cargo.toml --lib --release --verbose --target=wasm32-unknown-unknown

cargo build --manifest-path ./node/Cargo.toml --release --verbose

# cp ./target/x86_64-pc-windows-gnu/release/everywhere.exe ./out/everywhere.exe
# cp ./target/x86_64-unknown-linux-gnu/release/everywhere ./out/everywhere
# cp ./wasm/target/wasm32-unknown-unknown/release/everywhere.wasm ./out/everywhere.wasm
# cp ./node/target/release/node.dll ./out/everywhere.node
