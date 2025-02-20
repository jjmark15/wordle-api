FROM lukemathwalker/cargo-chef:latest-rust-1.61 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin word-guessing-game-api

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR app

EXPOSE 3030

COPY server-ft.yml .

ENV RUST_BACKTRACE=1

COPY --from=builder /app/target/release/word-guessing-game-api /usr/local/bin
ENTRYPOINT ["/usr/local/bin/word-guessing-game-api", "-c", "./server-ft.yml"]