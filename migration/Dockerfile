# ビルドステージ
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# ワークスペース構造を保持しつつ必要なファイルをコピー
COPY Cargo.toml ./
COPY migration/Cargo.toml ./migration/
COPY entity/Cargo.toml ./entity/
COPY api/Cargo.toml ./api/
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
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path migration/Cargo.toml

# ワークスペースのCargo.tomlをコピー
COPY Cargo.toml ./
COPY migration/Cargo.toml ./migration/
COPY entity/Cargo.toml ./entity/

# entityのソースコードをコピー（migrationが依存している）
COPY entity/src ./entity/src

# migrationのソースコードをコピー
COPY migration/src ./migration/src

RUN cargo build --release --manifest-path migration/Cargo.toml

# 実行ステージ
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/migration /usr/local/bin/migration

CMD ["migration", "up"]
