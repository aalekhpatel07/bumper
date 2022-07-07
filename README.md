# Bumper

[![Cargo (full)](https://github.com/aalekhpatel07/bumper/actions/workflows/cargo.yml/badge.svg)](https://github.com/aalekhpatel07/bumper/actions/workflows/cargo.yml)

## Running (from a release)

Unzip the contents of the release into a root directory where your web server will serve static files from.

```sh
unzip Bumper-v<version>.zip -d ~/bumper
```

Serve the directory with a static file server. For example `python3 -m http.server`

```sh
python3 -m http.server --bind 0.0.0.0 8000 --directory ~/bumper
```

## Developing

### Docker

You may use any `rust` base image with `wasm-pack` installed in it. Here, we use `aalekhpatel07/rust:1.0`

1. A very simple `Dockerfile` could look something like:

```Dockerfile
FROM aalekhpatel07/rust:1.0 as base
COPY . /app
RUN wasm-pack build --target web --release --out-dir /app/public/web

FROM python:3 as server
EXPOSE 8000
COPY --from=base /app/public /app
ENTRYPOINT ["python3", "-m", "http.server", "--bind", "0.0.0.0", "8000", "--directory", "/app"]
```

2. Build the image

```sh
docker build -f Dockerfile -t bumper:v<version> .
```

3. Run the image and bind the port to local machine.

```sh
docker run -d -p 8000:8000 bumper:v<version>
```

4. Navigate to `http://localhost:8000` to access the web app.

### Local

You may need to install wasm-pack with `cargo install wasm-pack` first.

1. Build the static files for the project.

```sh
wasm-pack build --target web --release --out-dir ./public/web
```
2. Serve the `./public` folder with a static file server.

```sh
python3 -m http.server --bind 0.0.0.0 8000 --directory ./public
```

3. Navigate to `http://localhost:8000` from a browser to view the app.
