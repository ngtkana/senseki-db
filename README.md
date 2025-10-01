# スマブラSP 戦績管理システム

スマブラSPの戦績を記録・分析するWebアプリケーション

## 🚀 クイックスタート

### 開発環境セットアップ

```bash
# データベース起動
docker compose up -d

# マイグレーション実行
cargo run --manifest-path migration/Cargo.toml

# Webアプリ起動
cd app
trunk serve --open
```

### デモデータで動作確認

```bash
DEMO_MODE=true cargo run --manifest-path migration/Cargo.toml
```

## 📁 プロジェクト構造

```
senseki-db/
├── app/          # Leptosアプリ（フロントエンド）
├── entity/       # SeaORM Entity
├── migration/    # データベースマイグレーション
└── docs/         # 開発ドキュメント
```

## 📚 ドキュメント

詳細なドキュメントは`docs/`ディレクトリを参照：

- [開発計画](docs/DEVELOPMENT_PLAN_V2.md)
- [クイックスタート](docs/QUICKSTART.md)
- [デモ実行ガイド](docs/README_DEMO.md)

## 🛠️ 技術スタック

- **フロントエンド**: Leptos (Rust)
- **データベース**: PostgreSQL
- **ORM**: SeaORM
- **開発ツール**: Trunk

## 📝 機能

- セッション管理
- マッチ記録（相手キャラ、勝敗、GSP、コメント）
- 戦績統計
- キャラクター別分析

## 🔧 開発

```bash
# Webアプリ開発
cd app
trunk serve

# データベース操作
cargo run --manifest-path migration/Cargo.toml
