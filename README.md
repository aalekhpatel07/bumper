# Car Driving Mechanics

## Building and running

### Compile the project into Wasm.

You may need to install wasm-pack with `cargo install wasm-pack` first.

```sh
wasm-pack build --target web --release --out-dir ./public/web
```

### Serve the `./public` folder statically.

```sh
python3 -m http.server --bind 0.0.0.0 8000 --directory ./public
```

Navigate to `http://localhost:8000` from a browser to view the app.