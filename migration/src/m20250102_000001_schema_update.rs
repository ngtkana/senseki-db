use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. charactersにname_en追加
        manager
            .alter_table(
                Table::alter()
                    .table(Characters::Table)
                    .add_column(
                        ColumnDef::new(Characters::NameEn)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // 2. sessionsにtitle追加
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(ColumnDef::new(Sessions::Title).string_len(100))
                    .to_owned(),
            )
            .await?;

        // 3. gsp_recordsテーブル作成
        manager
            .create_table(
                Table::create()
                    .table(GspRecords::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GspRecords::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GspRecords::SessionId).integer().not_null())
                    .col(ColumnDef::new(GspRecords::MatchOrder).integer().not_null())
                    .col(ColumnDef::new(GspRecords::Gsp).integer().not_null())
                    .col(ColumnDef::new(GspRecords::Note).text())
                    .col(
                        ColumnDef::new(GspRecords::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_gsp_records_session")
                            .from(GspRecords::Table, GspRecords::SessionId)
                            .to(Sessions::Table, Sessions::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // 4. gsp_recordsにユニーク制約追加
        manager
            .create_index(
                Index::create()
                    .name("idx_gsp_records_session_order")
                    .table(GspRecords::Table)
                    .col(GspRecords::SessionId)
                    .col(GspRecords::MatchOrder)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // 5. 既存データ移行（matchesのGSP → gsp_records）
        // この部分は手動で実行するか、データがある場合のみ実行
        // 今回は新規テーブルなのでスキップ

        // 6. matchesからGSPカラム削除
        manager
            .alter_table(
                Table::alter()
                    .table(Matches::Table)
                    .drop_column(Matches::GspBefore)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Matches::Table)
                    .drop_column(Matches::GspAfter)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Rollback処理
        manager
            .alter_table(
                Table::alter()
                    .table(Matches::Table)
                    .add_column(ColumnDef::new(Matches::GspBefore).integer())
                    .add_column(ColumnDef::new(Matches::GspAfter).integer())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(GspRecords::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_column(Sessions::Title)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Characters::Table)
                    .drop_column(Characters::NameEn)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Characters {
    Table,
    NameEn,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    Title,
}

#[derive(DeriveIden)]
enum Matches {
    Table,
    GspBefore,
    GspAfter,
}

#[derive(DeriveIden)]
enum GspRecords {
    Table,
    Id,
    SessionId,
    MatchOrder,
    Gsp,
    Note,
    CreatedAt,
}
