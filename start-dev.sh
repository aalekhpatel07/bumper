#!/usr/bin/sh

# Build the wasm project.
wasm-pack build --target web --release --out-dir ./public/web
