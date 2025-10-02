# ビルドステージ
FROM rust:1.90 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# trunkとwasm-bindgen-cliをインストール
RUN cargo install trunk wasm-bindgen-cli && \
    rustup target add wasm32-unknown-unknown

COPY --from=planner /app/recipe.json recipe.json
# 依存関係をビルド（キャッシュされる）
RUN cargo chef cook --release --recipe-path recipe.json --manifest-path app/Cargo.toml --target wasm32-unknown-unknown
# ソースコードをコピーしてtrunkでビルド
COPY . .
WORKDIR /app/app
RUN trunk build --release

# 実行ステージ - nginx
FROM nginx:alpine

# nginxの設定ファイルをコピー
COPY --from=builder /app/app/dist /usr/share/nginx/html

# カスタムnginx設定（SPAのルーティング対応）
RUN echo 'server { \
    listen 80; \
    server_name localhost; \
    root /usr/share/nginx/html; \
    index index.html; \
    location / { \
    try_files $uri $uri/ /index.html; \
    } \
    }' > /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
