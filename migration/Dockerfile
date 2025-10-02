# ビルドステージ
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# 依存関係をビルド（キャッシュされる）
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path migration/Cargo.toml
# ソースコードをコピーしてビルド
COPY . .
RUN cargo build --release --manifest-path migration/Cargo.toml

# 実行ステージ
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/migration /usr/local/bin/migration

CMD ["migration", "up"]
