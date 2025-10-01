# スマブラSP 戦績管理データベース

スマブラSPの対戦記録を管理するためのデータベース設計プロジェクト。

## データベース設計

### テーブル構成

1. **characters** - キャラクターマスタ
   - スマブラSPの全キャラクター（89体）を管理
   
2. **sessions** - セッション（1日の対戦記録）
   - 日付、メモ（意気込みなど）を記録
   
3. **matches** - マッチ（個別の対戦）
   - 使用キャラ、相手キャラ、勝敗、GSP、コメントを記録

### 主な機能

- ✅ キャラクター別の戦績管理
- ✅ 世界戦闘力（GSP）の記録（任意）
- ✅ セッションごとのメモ・意気込み
- ✅ マッチごとのコメント（ラグ、1スト、切断など）
- ✅ 順序保証（match_order）
- ✅ 自動タイムスタンプ管理

## 開発環境セットアップ

### 前提条件

- PostgreSQL 14以上
- Rust 1.70以上
- Docker（オプション）

### ローカル開発環境

#### 1. PostgreSQLのセットアップ

**Option A: Dockerを使用（推奨）**

```bash
# PostgreSQLコンテナを起動
docker run --name senseki-postgres \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_DB=senseki \
  -p 5432:5432 \
  -d postgres:16

# 接続確認
docker exec -it senseki-postgres psql -U postgres -d senseki
```

**Option B: ローカルインストール**

```bash
# Ubuntu/Debian
sudo apt install postgresql postgresql-contrib

# macOS
brew install postgresql@16

# データベース作成
createdb senseki
```

#### 2. スキーマの適用

```bash
# スキーマを適用
psql -U postgres -d senseki -f schema.sql

# 接続確認
psql -U postgres -d senseki -c "\dt"
```

#### 3. Rustプロジェクトのセットアップ

```bash
# プロジェクト初期化
cargo init --name senseki-db

# SeaORMの依存関係を追加
cargo add sea-orm --features sqlx-postgres,runtime-tokio-native-tls,macros
cargo add tokio --features full
cargo add dotenvy

# SeaORM CLIのインストール
cargo install sea-orm-cli
```

#### 4. 環境変数の設定

`.env`ファイルを作成：

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/senseki
```

#### 5. マイグレーションの作成

```bash
# マイグレーションディレクトリを初期化
sea-orm-cli migrate init

# 既存のスキーマからマイグレーションを生成
# （手動で migration/src/m20250102_000001_create_tables.rs を作成）

# マイグレーション実行
sea-orm-cli migrate up

# Entity生成
sea-orm-cli generate entity -o src/entity
```

## 本番環境（Google Cloud SQL）

### セットアップ手順

1. **Cloud SQLインスタンスの作成**

```bash
gcloud sql instances create senseki-db \
  --database-version=POSTGRES_16 \
  --tier=db-f1-micro \
  --region=asia-northeast1
```

2. **データベースの作成**

```bash
gcloud sql databases create senseki --instance=senseki-db
```

3. **接続設定**

```bash
# Cloud SQL Proxyを使用
cloud-sql-proxy senseki-project:asia-northeast1:senseki-db
```

4. **スキーマの適用**

```bash
psql -h 127.0.0.1 -U postgres -d senseki -f schema.sql
```

## 開発フロー

### ローカル開発

1. ローカルPostgreSQLで開発・テスト
2. マイグレーションファイルで変更を管理
3. 動作確認後、本番環境にデプロイ

### データ移行

既存のテキストファイル（`../../tasks/senseki/notes/`）からのデータ移行スクリプトを作成予定。

## 技術スタック

- **Database**: PostgreSQL 16
- **ORM**: SeaORM
- **Language**: Rust
- **Cloud**: Google Cloud SQL
- **Migration**: SeaORM CLI

## 次のステップ

- [ ] キャラクターマスタの初期データ作成
- [ ] SeaORMマイグレーションファイルの作成
- [ ] CRUD APIの実装
- [ ] 既存データのインポートスクリプト
- [ ] 統計・分析機能の実装
- [ ] エクスポート機能（CSV/テキスト）

## ライセンス

MIT
