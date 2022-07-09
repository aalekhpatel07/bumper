#!/bin/sh

cd /app
wasm-pack build --release --target web --out-dir /app/public/web --verbose
