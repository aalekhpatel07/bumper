FROM rust

RUN apt update -y && apt upgrade -y
RUN rustup update
RUN rustup component add clippy
RUN rustup component add rustfmt
RUN cargo install wasm-pack cargo-nextest cargo-tarpaulin run-when
CMD ["/bin/sleep", "infinity"]
