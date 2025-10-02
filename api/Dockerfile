# ビルドステージ
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# ワークスペース構造を保持しつつ必要なファイルをコピー
COPY Cargo.toml ./
COPY api/Cargo.toml ./api/
COPY entity/Cargo.toml ./entity/
COPY migration/Cargo.toml ./migration/
COPY app/Cargo.toml ./app/
# ダミーのソースファイルを作成（cargo metadataが動作するため）
RUN mkdir -p entity/src api/src app/src migration/src && \
    echo "fn main() {}" > entity/src/lib.rs && \
    echo "fn main() {}" > api/src/lib.rs && \
    echo "fn main() {}" > api/src/main.rs && \
    echo "fn main() {}" > app/src/main.rs && \
    echo "fn main() {}" > migration/src/lib.rs && \
    echo "fn main() {}" > migration/src/main.rs
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# 依存関係をビルド（キャッシュされる）
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path api/Cargo.toml

# ワークスペースのCargo.tomlをコピー
COPY Cargo.toml ./
COPY api/Cargo.toml ./api/
COPY entity/Cargo.toml ./entity/

# entityのソースコードをコピー（apiが依存している）
COPY entity/src ./entity/src

# apiのソースコードをコピー
COPY api/src ./api/src

RUN cargo build --release --manifest-path api/Cargo.toml

# 実行ステージ
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 curl && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/senseki-api /usr/local/bin/senseki-api

EXPOSE 3000

CMD ["senseki-api"]
