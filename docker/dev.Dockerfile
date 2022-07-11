FROM aalekhpatel07/rust:1.0 as base
EXPOSE 8000
WORKDIR /app
COPY . .

RUN apt-get update -y
RUN apt-get install python3.9

RUN chmod +x scripts/*

RUN cd bumper-web && wasm-pack build --target web --release --out-dir /app/public/web

CMD ["/bin/bash", "/app/scripts/entrypoint-dev.sh"]
