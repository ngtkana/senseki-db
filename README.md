# スマブラSP 戦績管理システム

スマブラSPの戦績を記録・分析するフルスタックWebアプリケーション

## 🎯 概要

- **フロントエンド**: Leptos 0.8 (Rust WASM)
- **バックエンド**: Axum 0.8 REST API
- **データベース**: PostgreSQL + SeaORM
- **開発ツール**: Trunk, cargo

## 🚀 クイックスタート

### 1. データベース起動

```bash
# PostgreSQL起動（Docker）
docker run -d \
  --name senseki-postgres \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=senseki \
  postgres:16
```

### 2. マイグレーション実行

```bash
# キャラクターデータも投入
DEMO_MODE=true cargo run --manifest-path migration/Cargo.toml
```

### 3. APIサーバー起動

```bash
cd api
cargo run
# → http://127.0.0.1:3000
```

### 4. Webアプリ起動

```bash
cd app
trunk serve
# → http://127.0.0.1:8080
```

## 📁 プロジェクト構造

```
senseki-db/
├── entity/       # SeaORM Entity（データモデル）
├── migration/    # データベースマイグレーション
├── api/          # Axum REST APIサーバー
├── app/          # Leptos Webアプリ（フロントエンド）
└── docs/         # ドキュメント
    ├── PROGRESS.md      # 開発進捗
    └── DB_CHECK.md      # データベース確認方法
```

## 🛠️ 技術スタック

### フロントエンド
- **Leptos 0.8**: Rust製リアクティブWebフレームワーク
- **WASM**: WebAssemblyでブラウザ実行
- **Trunk**: WASMビルドツール
- **gloo-net**: HTTP通信

### バックエンド
- **Axum 0.8**: 高速Webフレームワーク
- **SeaORM 1.1**: 型安全ORM
- **PostgreSQL 16**: リレーショナルデータベース
- **Tower-HTTP**: CORS対応

## 📊 データベース設計

### テーブル構成

```sql
-- キャラクター（全89体）
characters (id, name)

-- セッション（プレイ日ごと）
sessions (id, session_date, notes)

-- マッチ（試合記録）
matches (
  id, session_id, character_id, opponent_character_id,
  result, match_order, gsp_before, gsp_after, comment
)
```

## ✨ 実装済み機能

### セッション管理
- ✅ セッション一覧表示
- ✅ セッション作成
- ✅ 今日の戦績表示（試合数、勝敗、勝率）

### マッチ記録
- ✅ マッチ記録フォーム
- ✅ キャラクター選択（全89体）
- ✅ 勝敗記録
- ✅ GSP記録（任意）
- ✅ コメント記録（任意）

### API
- ✅ REST API（全9エンドポイント）
- ✅ CORS対応
- ✅ ログ出力

## 🔍 データベース確認

### psqlで確認

```bash
psql postgres://postgres:password@localhost:5432/senseki

# セッション一覧
SELECT * FROM sessions ORDER BY session_date DESC;

# マッチ一覧（キャラクター名付き）
SELECT 
    m.id, s.session_date,
    c1.name as character,
    c2.name as opponent,
    m.result, m.gsp_before, m.gsp_after
FROM matches m
JOIN sessions s ON m.session_id = s.id
JOIN characters c1 ON m.character_id = c1.id
JOIN characters c2 ON m.opponent_character_id = c2.id
ORDER BY s.session_date DESC, m.match_order;
```

### curlでAPI確認

```bash
# セッション一覧
curl http://127.0.0.1:3000/api/sessions | jq

# キャラクター一覧
curl http://127.0.0.1:3000/api/characters | jq

# セッション作成
curl -X POST http://127.0.0.1:3000/api/sessions \
  -H "Content-Type: application/json" \
  -d '{"session_date": "2025-10-02", "notes": "今日の目標"}' | jq
```

詳細は `docs/DB_CHECK.md` を参照。

## 🔧 開発コマンド

```bash
# コード品質チェック
cargo check
cargo clippy

# テスト実行
cargo test

# フォーマット
cargo fmt

# Webアプリ開発（ホットリロード）
cd app && trunk serve

# APIサーバー開発
cd api && cargo watch -x run
```

## 📝 今後の拡張案

- [ ] マッチ一覧表示
- [ ] セッション詳細ページ
- [ ] キャラクター別勝率統計
- [ ] 期間別集計
- [ ] グラフ表示
- [ ] データエクスポート（CSV）
- [ ] マッチ編集・削除機能

## 📚 ドキュメント

- [開発進捗](docs/PROGRESS.md)
- [データベース確認方法](docs/DB_CHECK.md)

## 🤝 コントリビューション

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## 📄 ライセンス

MIT License
