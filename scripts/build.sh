#!/bin/sh

cd /app/bumper-web
wasm-pack build --release --target web --out-dir /app/public/web --verbose
