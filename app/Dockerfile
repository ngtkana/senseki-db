# ビルドステージ
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
# 必要最小限のファイルのみをコピー
COPY Cargo.toml ./
COPY app/Cargo.toml ./app/
COPY entity/Cargo.toml ./entity/
COPY migration/Cargo.toml ./migration/
COPY api/Cargo.toml ./api/
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
# trunkとwasm-bindgen-cliをインストール
RUN cargo install trunk wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown

COPY --from=planner /app/recipe.json recipe.json
# 依存関係をビルド（キャッシュされる）
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path app/Cargo.toml --target wasm32-unknown-unknown

# 静的アセット（変更頻度が低い）を先にコピー
COPY app/public ./app/public
COPY app/index.html app/style.css ./app/

# Cargo.tomlをコピー
COPY Cargo.toml ./
COPY app/Cargo.toml ./app/

# ソースコードをコピー（変更頻度が高い）
COPY app/src ./app/src

# trunkでビルド
WORKDIR /app/app
RUN trunk build --release

# 実行ステージ - nginx
FROM nginx:alpine

# ビルド成果物をコピー
COPY --from=builder /app/app/dist /usr/share/nginx/html

# カスタムnginx設定をコピー
COPY app/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
