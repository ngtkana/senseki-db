use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // まず全てのキャラクターにユニークな仮の値を設定
        db.execute_unprepared("UPDATE characters SET name_en = 'temp_' || id::text")
            .await?;

        // キャラクター英語名を更新
        let updates = vec![
            ("マリオ", "Mario"),
            ("ドンキーコング", "Donkey Kong"),
            ("リンク", "Link"),
            ("サムス", "Samus"),
            ("ダークサムス", "Dark Samus"),
            ("ヨッシー", "Yoshi"),
            ("カービィ", "Kirby"),
            ("フォックス", "Fox"),
            ("ピカチュウ", "Pikachu"),
            ("ルイージ", "Luigi"),
            ("ネス", "Ness"),
            ("キャプテン・ファルコン", "Captain Falcon"),
            ("プリン", "Jigglypuff"),
            ("ピーチ", "Peach"),
            ("デイジー", "Daisy"),
            ("クッパ", "Bowser"),
            ("アイスクライマー", "Ice Climbers"),
            ("シーク", "Sheik"),
            ("ゼルダ", "Zelda"),
            ("ドクターマリオ", "Dr. Mario"),
            ("ピチュー", "Pichu"),
            ("ファルコ", "Falco"),
            ("マルス", "Marth"),
            ("ルキナ", "Lucina"),
            ("こどもリンク", "Young Link"),
            ("ガノンドロフ", "Ganondorf"),
            ("ミュウツー", "Mewtwo"),
            ("ロイ", "Roy"),
            ("クロム", "Chrom"),
            ("Mr.ゲーム&ウォッチ", "Mr. Game & Watch"),
            ("メタナイト", "Meta Knight"),
            ("ピット", "Pit"),
            ("ブラックピット", "Dark Pit"),
            ("ゼロスーツサムス", "Zero Suit Samus"),
            ("ワリオ", "Wario"),
            ("スネーク", "Snake"),
            ("アイク", "Ike"),
            ("ポケモントレーナー", "Pokemon Trainer"),
            ("ディディーコング", "Diddy Kong"),
            ("リュカ", "Lucas"),
            ("ソニック", "Sonic"),
            ("デデデ", "King Dedede"),
            ("ピクミン&オリマー", "Olimar"),
            ("ルカリオ", "Lucario"),
            ("ロボット", "R.O.B."),
            ("トゥーンリンク", "Toon Link"),
            ("ウルフ", "Wolf"),
            ("むらびと", "Villager"),
            ("ロックマン", "Mega Man"),
            ("Wii Fit トレーナー", "Wii Fit Trainer"),
            ("ロゼッタ&チコ", "Rosalina & Luma"),
            ("リトル・マック", "Little Mac"),
            ("ゲッコウガ", "Greninja"),
            ("Miiファイター(格闘)", "Mii Brawler"),
            ("Miiファイター(剣術)", "Mii Swordfighter"),
            ("Miiファイター(射撃)", "Mii Gunner"),
            ("パルテナ", "Palutena"),
            ("パックマン", "Pac-Man"),
            ("ルフレ", "Robin"),
            ("シュルク", "Shulk"),
            ("クッパJr.", "Bowser Jr."),
            ("ダックハント", "Duck Hunt"),
            ("リュウ", "Ryu"),
            ("ケン", "Ken"),
            ("クラウド", "Cloud"),
            ("カムイ", "Corrin"),
            ("ベヨネッタ", "Bayonetta"),
            ("インクリング", "Inkling"),
            ("リドリー", "Ridley"),
            ("シモン", "Simon"),
            ("リヒター", "Richter"),
            ("キングクルール", "King K. Rool"),
            ("しずえ", "Isabelle"),
            ("ガオガエン", "Incineroar"),
            ("パックンフラワー", "Piranha Plant"),
            ("ジョーカー", "Joker"),
            ("勇者", "Hero"),
            ("バンジョー&カズーイ", "Banjo & Kazooie"),
            ("テリー", "Terry"),
            ("ベレト/ベレス", "Byleth"),
            ("ミェンミェン", "Min Min"),
            ("スティーブ/アレックス", "Steve"),
            ("セフィロス", "Sephiroth"),
            ("ホムラ/ヒカリ", "Pyra/Mythra"),
            ("カズヤ", "Kazuya"),
            ("ソラ", "Sora"),
        ];

        for (name_jp, name_en) in updates {
            db.execute_unprepared(&format!(
                "UPDATE characters SET name_en = '{}' WHERE name = '{}'",
                name_en, name_jp
            ))
            .await?;
        }

        // name_enにユニーク制約を追加
        manager
            .create_index(
                Index::create()
                    .name("idx_characters_name_en")
                    .table(Characters::Table)
                    .col(Characters::NameEn)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_characters_name_en")
                    .table(Characters::Table)
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
