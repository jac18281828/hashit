FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=hashit
WORKDIR /workspaces/${PROJECT}

COPY --chown=rust:rust . .

ENV USER=rust
USER rust

RUN yamlfmt -lint .github/workflows/*.yml

ENV CARGO_INCREMENTAL=${CARGO_INCREMENTAL:-1}
RUN cargo check --all-features
RUN cargo fmt --check --all
RUN cargo clippy --all-features --no-deps -- -D warnings
RUN cargo test --workspace --all-features
RUN CARGO_TARGET_DIR=/workspaces/${PROJECT}/target cargo install --path . --root ~${USER}/.cargo/
RUN valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ~${USER}/.cargo/bin/hashit --help

ENV RUST_LOG=info
CMD cargo run --package=resolver -- --host 0.0.0.0 --port 8080
