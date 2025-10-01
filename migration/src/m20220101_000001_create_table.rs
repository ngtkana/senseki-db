use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // キャラクターマスタテーブル
        manager
            .create_table(
                Table::create()
                    .table(Character::Table)
                    .if_not_exists()
                    .col(pk_auto(Character::Id))
                    .col(string_len(Character::Name, 50).not_null().unique_key())
                    .col(timestamp(Character::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Character::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // セッションテーブル
        manager
            .create_table(
                Table::create()
                    .table(Session::Table)
                    .if_not_exists()
                    .col(pk_auto(Session::Id))
                    .col(date(Session::SessionDate).not_null())
                    .col(text_null(Session::Notes))
                    .col(timestamp(Session::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Session::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // マッチテーブル
        manager
            .create_table(
                Table::create()
                    .table(Match::Table)
                    .if_not_exists()
                    .col(pk_auto(Match::Id))
                    .col(integer(Match::SessionId).not_null())
                    .col(integer(Match::CharacterId).not_null())
                    .col(integer(Match::OpponentCharacterId).not_null())
                    .col(string_len(Match::Result, 10).not_null())
                    .col(integer(Match::MatchOrder).not_null())
                    .col(integer_null(Match::GspBefore))
                    .col(integer_null(Match::GspAfter))
                    .col(text_null(Match::Comment))
                    .col(timestamp(Match::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(Match::UpdatedAt).default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_match_session")
                            .from(Match::Table, Match::SessionId)
                            .to(Session::Table, Session::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_match_character")
                            .from(Match::Table, Match::CharacterId)
                            .to(Character::Table, Character::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_match_opponent_character")
                            .from(Match::Table, Match::OpponentCharacterId)
                            .to(Character::Table, Character::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // CHECK制約（result）- テーブル作成後に追加
        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE "matches" ADD CONSTRAINT check_result CHECK (result IN ('win', 'loss'))"#,
            )
            .await?;

        // UNIQUE制約（session_id, match_order）
        manager
            .create_index(
                Index::create()
                    .name("idx_match_session_order")
                    .table(Match::Table)
                    .col(Match::SessionId)
                    .col(Match::MatchOrder)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // インデックス作成
        manager
            .create_index(
                Index::create()
                    .name("idx_sessions_date")
                    .table(Session::Table)
                    .col(Session::SessionDate)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_matches_session")
                    .table(Match::Table)
                    .col(Match::SessionId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_matches_character")
                    .table(Match::Table)
                    .col(Match::CharacterId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_matches_opponent")
                    .table(Match::Table)
                    .col(Match::OpponentCharacterId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_matches_created_at")
                    .table(Match::Table)
                    .col(Match::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // updated_at自動更新関数
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE OR REPLACE FUNCTION update_updated_at_column()
                RETURNS TRIGGER AS $$
                BEGIN
                  NEW.updated_at = CURRENT_TIMESTAMP;
                  RETURN NEW;
                END;
                $$ LANGUAGE plpgsql;
                "#,
            )
            .await?;

        // トリガー作成
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_characters_updated_at
                  BEFORE UPDATE ON characters
                  FOR EACH ROW
                  EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_sessions_updated_at
                  BEFORE UPDATE ON sessions
                  FOR EACH ROW
                  EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TRIGGER update_matches_updated_at
                  BEFORE UPDATE ON matches
                  FOR EACH ROW
                  EXECUTE FUNCTION update_updated_at_column();
                "#,
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // トリガー削除
        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_matches_updated_at ON matches")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_sessions_updated_at ON sessions")
            .await?;

        manager
            .get_connection()
            .execute_unprepared("DROP TRIGGER IF EXISTS update_characters_updated_at ON characters")
            .await?;

        // 関数削除
        manager
            .get_connection()
            .execute_unprepared("DROP FUNCTION IF EXISTS update_updated_at_column()")
            .await?;

        // テーブル削除（外部キー制約があるので順序重要）
        manager
            .drop_table(Table::drop().table(Match::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Session::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Character::Table).to_owned())
            .await?;

        Ok(())
    }
}

enum Character {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "characters",
                Self::Id => "id",
                Self::Name => "name",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}

enum Session {
    Table,
    Id,
    SessionDate,
    Notes,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Session {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "sessions",
                Self::Id => "id",
                Self::SessionDate => "session_date",
                Self::Notes => "notes",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}

enum Match {
    Table,
    Id,
    SessionId,
    CharacterId,
    OpponentCharacterId,
    Result,
    MatchOrder,
    GspBefore,
    GspAfter,
    Comment,
    CreatedAt,
    UpdatedAt,
}

impl Iden for Match {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(
            s,
            "{}",
            match self {
                Self::Table => "matches",
                Self::Id => "id",
                Self::SessionId => "session_id",
                Self::CharacterId => "character_id",
                Self::OpponentCharacterId => "opponent_character_id",
                Self::Result => "result",
                Self::MatchOrder => "match_order",
                Self::GspBefore => "gsp_before",
                Self::GspAfter => "gsp_after",
                Self::Comment => "comment",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        )
        .unwrap();
    }
}
