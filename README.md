# スマブラSP 戦績管理

スマブラSPの戦績を記録・管理するWebアプリ

## セットアップ

### 1. データベース起動

```bash
docker run -d \
  --name senseki-postgres \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=senseki \
  postgres:16
```

### 2. マイグレーション実行

```bash
cargo run --manifest-path migration/Cargo.toml
```

### 3. APIサーバー起動

```bash
cd api && cargo run
# → http://127.0.0.1:3000
```

### 4. Webアプリ起動

```bash
cd app && trunk serve
# → http://127.0.0.1:8080
```

## 使い方

### セッション作成
1. 左サイドバーの「+ セッション」をクリック
2. 日付とタイトルを入力

### マッチ記録
1. 「+ マッチを追加」をクリック
2. 自キャラと相手キャラを入力（日本語名、英語名、内部キーで検索可能）
3. 勝敗ボタンをクリック（トグル式）
4. 相手キャラ選択で自動確定

### マッチ削除
- 単一削除: チェックボックスをクリック
- 複数削除: Shift/Ctrlで範囲選択・複数選択 → 「選択を削除」ボタン

## 技術スタック

- **フロントエンド**: Leptos 0.8 (Rust WASM)
- **バックエンド**: Axum 0.8
- **データベース**: PostgreSQL 16 + SeaORM 1.1

## プロジェクト構造

```
senseki-db/
├── entity/       # データモデル
├── migration/    # マイグレーション
├── api/          # REST API
└── app/          # Webアプリ
```

## データベース確認

```bash
psql postgres://postgres:password@localhost:5432/senseki

# マッチ一覧
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
```
