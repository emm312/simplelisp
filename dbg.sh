#!/bin/sh
wasm-pack build --target web --features wasm
python3 -m http.server