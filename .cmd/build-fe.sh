#!/bin/sh
set -e
cd skatt-fe
RUSTFLAGS="-C lto=fat -C embed-bitcode=yes -C codegen-units=1 -C strip=symbols" trunk build --release --minify
FILE=$(find ./dist -name '*.wasm')
wasm-opt --strip-debug -Oz $FILE -o $FILE
