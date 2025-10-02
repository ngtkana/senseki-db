# スマブラSP 戦績管理

スマブラSPの戦績を記録・管理するWebアプリ

## セットアップ

```bash
cp .env.example .env
docker compose up -d
```

起動後: http://localhost:8080

## 使い方

### セッション作成
左サイドバー「+ セッション」→ 日付とタイトルを入力

### マッチ記録
「+ マッチを追加」→ 自キャラと相手キャラを入力 → 勝敗をクリック

### マッチ削除
チェックボックスで選択 → 「選択を削除」（Shift/Ctrl で範囲・複数選択可）

## 技術スタック

- **フロントエンド**: Leptos 0.8 (Rust WASM)
- **バックエンド**: Axum 0.8
- **データベース**: PostgreSQL 16 + SeaORM 1.1
- **Rust**: 1.90.0

## プロジェクト構造

```
senseki-db/
├── entity/       # データモデル
├── migration/    # マイグレーション
├── api/          # REST API
└── app/          # Webアプリ
```

## 開発

詳細は `DOCKER.md` を参照

### 個別起動

```bash
# データベース
docker run -d --name senseki-postgres -p 5432:5432 \
  -e POSTGRES_PASSWORD=password -e POSTGRES_DB=senseki postgres:16

# マイグレーション
cargo run --manifest-path migration/Cargo.toml

# API
cd api && cargo run

# フロントエンド
cd app && trunk serve
```

### データベース確認

```bash
psql postgres://postgres:password@localhost:5432/senseki
```

```sql
SELECT 
    m.match_order, s.session_date,
    c1.name as character,
    c2.name as opponent,
    m.result
FROM matches m
JOIN sessions s ON m.session_id = s.id
JOIN characters c1 ON m.character_id = c1.id
JOIN characters c2 ON m.opponent_character_id = c2.id
ORDER BY s.session_date DESC, m.match_order;
