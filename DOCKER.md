# Docker構成ガイド

## 概要

このプロジェクトは、Docker Composeを使用して以下のサービスを管理します：
- PostgreSQL データベース
- マイグレーション（初回起動時のみ）
- APIサーバー（Rust/Axum）
- フロントエンド（Rust/Leptos + nginx）

## セットアップ

### 1. 環境変数の設定

`.env.example`をコピーして`.env`ファイルを作成：

```bash
cp .env.example .env
```

`.env`ファイルを編集して、本番環境では必ず強力なパスワードに変更してください：

```env
POSTGRES_PASSWORD=your_secure_password_here
```

### 2. コンテナの起動

```bash
# すべてのサービスを起動
docker compose up -d

# ログを確認
docker compose logs -f

# 特定のサービスのログを確認
docker compose logs -f api
```

### 3. コンテナの停止

```bash
# すべてのサービスを停止
docker compose down

# ボリュームも削除（データベースのデータも削除される）
docker compose down -v
```

## 開発時の注意事項

### ポートバインディング

開発環境では、すべてのポートが`127.0.0.1`（localhost）にバインドされています：
- PostgreSQL: `127.0.0.1:5432`
- API: `127.0.0.1:3000`
- フロントエンド: `127.0.0.1:8080`

これにより、外部ネットワークからのアクセスを防ぎます。

### ヘルスチェック

各サービスにはヘルスチェックが設定されています：
- **db**: `pg_isready`コマンドで確認
- **api**: `/api/characters`エンドポイントで確認
- **app**: nginxのルートパスで確認

ヘルスチェックにより、依存関係のあるサービスが正常に起動してから次のサービスが起動します。

### ビルドキャッシュの活用

Dockerfileは以下の最適化を実施しています：

1. **cargo-chef**: 依存関係のビルドをキャッシュ
2. **レイヤーの順序**: 変更頻度の低いファイルを先にコピー
3. **マルチステージビルド**: ビルド成果物のみを実行イメージに含める

ソースコードのみを変更した場合、依存関係のビルドはスキップされます。

## 本番環境への展開

### セキュリティチェックリスト

- [ ] `.env`ファイルで強力なパスワードを設定
- [ ] `.env`ファイルを`.gitignore`に追加（既に追加済み）
- [ ] ポートバインディングを適切に設定（必要に応じて`0.0.0.0`に変更）
- [ ] データベースポート（5432）を外部に公開しない
- [ ] HTTPS/TLSを設定（リバースプロキシ経由を推奨）
- [ ] リソース制限を設定（必要に応じて）

### 推奨される追加設定

本番環境では、以下の設定を追加することを推奨します：

```yaml
services:
  api:
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 512M
        reservations:
          cpus: '0.5'
          memory: 256M
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## トラブルシューティング

### コンテナが起動しない

```bash
# ログを確認
docker compose logs

# 特定のサービスを再起動
docker compose restart api

# すべてを再ビルド
docker compose up -d --build
```

### データベース接続エラー

1. データベースが起動しているか確認：
   ```bash
   docker compose ps
   ```

2. ヘルスチェックの状態を確認：
   ```bash
   docker compose ps
   # "healthy"と表示されるはず
   ```

3. 環境変数が正しく設定されているか確認：
   ```bash
   docker compose config
   ```

### ビルドが遅い

初回ビルドは時間がかかりますが、2回目以降はキャッシュが効きます。

キャッシュをクリアして再ビルドする場合：
```bash
docker compose build --no-cache
```

## 改善履歴

### 実施済みの改善

1. ✅ パスワードを環境変数に移行
2. ✅ ポートバインディングを`127.0.0.1`に変更（開発環境）
3. ✅ nginx設定を別ファイルに分離
4. ✅ ヘルスチェックを追加（api, app）
5. ✅ ビルドキャッシュの最適化

### 今後の改善案

- リソース制限の追加
- ログローテーションの設定
- 開発/本番環境の分離（docker-compose.dev.yml）
- 共通ベースイメージの作成
