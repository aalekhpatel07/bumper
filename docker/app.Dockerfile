FROM aalekhpatel07/rust:1.0 as base
COPY . /app

RUN chmod +x /app/scripts/*
WORKDIR /app
RUN wasm-pack build --target web --release --out-dir /app/public/web

FROM python:3 as server
EXPOSE 8000
COPY --from=base /app/public /app
CMD ["python3", "-m", "http.server", "--bind", "0.0.0.0", "8000", "--directory", "/app"]
