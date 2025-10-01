use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // スマブラSP 全キャラクターの初期データ
        let characters = [
            // ファイター番号順
            // https://www.smashbros.com/ja_JP/fighter/index.html
            "マリオ",
            "ドンキーコング",
            "リンク",
            "サムス",
            "ダークサムス",
            "ヨッシー",
            "カービィ",
            "フォックス",
            "ピカチュウ",
            "ルイージ",
            "ネス",
            "キャプテン・ファルコン",
            "プリン",
            "ピーチ",
            "デイジー",
            "クッパ",
            "アイスクライマー",
            "シーク",
            "ゼルダ",
            "ドクターマリオ",
            "ピチュー",
            "ファルコ",
            "マルス",
            "ルキナ",
            "こどもリンク",
            "ガノンドロフ",
            "ミュウツー",
            "ロイ",
            "クロム",
            "Mr.ゲーム&ウォッチ",
            "メタナイト",
            "ピット",
            "ブラックピット",
            "ゼロスーツサムス",
            "ワリオ",
            "スネーク",
            "アイク",
            "ポケモントレーナー",
            "ディディーコング",
            "リュカ",
            "ソニック",
            "デデデ",
            "ピクミン&オリマー",
            "ルカリオ",
            "ロボット",
            "トゥーンリンク",
            "ウルフ",
            "むらびと",
            "ロックマン",
            "Wii Fit トレーナー",
            "ロゼッタ&チコ",
            "リトル・マック",
            "ゲッコウガ",
            "Miiファイター(格闘)",
            "Miiファイター(剣術)",
            "Miiファイター(射撃)",
            "パルテナ",
            "パックマン",
            "ルフレ",
            "シュルク",
            "クッパJr.",
            "ダックハント",
            "リュウ",
            "ケン",
            "クラウド",
            "カムイ",
            "ベヨネッタ",
            "インクリング",
            "リドリー",
            "シモン",
            "リヒター",
            "キングクルール",
            "しずえ",
            "ガオガエン",
            "パックンフラワー",
            "ジョーカー",
            "勇者",
            "バンジョー&カズーイ",
            "テリー",
            "ベレト/ベレス",
            "ミェンミェン",
            "スティーブ/アレックス",
            "セフィロス",
            "ホムラ/ヒカリ",
            "カズヤ",
            "ソラ",
        ];

        for character in characters {
            let insert = Query::insert()
                .into_table(Character::Table)
                .columns([Character::Name])
                .values_panic([character.into()])
                .to_owned();

            manager.exec_stmt(insert).await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 全キャラクターデータを削除
        let delete = Query::delete().from_table(Character::Table).to_owned();

        manager.exec_stmt(delete).await?;

        Ok(())
    }
}

enum Character {
    Table,
    Name,
}

impl Iden for Character {
    fn unquoted(&self, s: &mut dyn std::fmt::Write) {
        write!(s, "{}", match self {
            Self::Table => "characters",
            Self::Name => "name",
        })
        .unwrap();
    }
}
