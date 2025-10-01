use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // fighter_keyカラム追加
        manager
            .alter_table(
                Table::alter()
                    .table(Characters::Table)
                    .add_column(
                        ColumnDef::new(Characters::FighterKey)
                            .string_len(50)
                            .not_null()
                            .default(""),
                    )
                    .to_owned(),
            )
            .await?;

        // まず全てのキャラクターにユニークな仮の値を設定
        db.execute_unprepared("UPDATE characters SET fighter_key = 'temp_key_' || id::text")
            .await?;

        // キャラクター情報を更新（CSVベース）
        let updates = vec![
            ("マリオ", "Mario", "mario"),
            ("ドンキーコング", "Donkey Kong", "donkey"),
            ("リンク", "Link", "link"),
            ("サムス", "Samus", "samus"),
            ("ダークサムス", "Dark Samus", "samusd"),
            ("ヨッシー", "Yoshi", "yoshi"),
            ("カービィ", "Kirby", "kirby"),
            ("フォックス", "Fox", "fox"),
            ("ピカチュウ", "Pikachu", "pikachu"),
            ("ルイージ", "Luigi", "luigi"),
            ("ネス", "Ness", "ness"),
            ("キャプテン・ファルコン", "Captain Falcon", "captain"),
            ("プリン", "Jigglypuff", "purin"),
            ("ピーチ", "Peach", "peach"),
            ("デイジー", "Daisy", "daisy"),
            ("クッパ", "Bowser", "koopa"),
            ("アイスクライマー", "Ice Climbers", "popo"),
            ("シーク", "Sheik", "sheik"),
            ("ゼルダ", "Zelda", "zelda"),
            ("ドクターマリオ", "Dr. Mario", "mariod"),
            ("ピチュー", "Pichu", "pichu"),
            ("ファルコ", "Falco", "falco"),
            ("マルス", "Marth", "marth"),
            ("ルキナ", "Lucina", "lucina"),
            ("こどもリンク", "Young Link", "younglink"),
            ("ガノンドロフ", "Ganondorf", "ganon"),
            ("ミュウツー", "Mewtwo", "mewtwo"),
            ("ロイ", "Roy", "roy"),
            ("クロム", "Chrom", "chrom"),
            ("Mr.ゲーム&ウォッチ", "Mr. Game & Watch", "gamewatch"),
            ("メタナイト", "Meta Knight", "metaknight"),
            ("ピット", "Pit", "pit"),
            ("ブラックピット", "Dark Pit", "pitb"),
            ("ゼロスーツサムス", "Zero Suit Samus", "szerosuit"),
            ("ワリオ", "Wario", "wario"),
            ("スネーク", "Snake", "snake"),
            ("アイク", "Ike", "ike"),
            ("ポケモントレーナー", "Pokemon Trainer", "pzenigame"),
            ("ディディーコング", "Diddy Kong", "diddy"),
            ("リュカ", "Lucas", "lucas"),
            ("ソニック", "Sonic", "sonic"),
            ("デデデ", "King Dedede", "dedede"),
            ("ピクミン&オリマー", "Olimar", "pikmin"),
            ("ルカリオ", "Lucario", "lucario"),
            ("ロボット", "R.O.B.", "robot"),
            ("トゥーンリンク", "Toon Link", "toonlink"),
            ("ウルフ", "Wolf", "wolf"),
            ("むらびと", "Villager", "murabito"),
            ("ロックマン", "Mega Man", "rockman"),
            ("Wii Fit トレーナー", "Wii Fit Trainer", "wiifit"),
            ("ロゼッタ&チコ", "Rosalina & Luma", "rosetta"),
            ("リトル・マック", "Little Mac", "littlemac"),
            ("ゲッコウガ", "Greninja", "gekkouga"),
            ("Miiファイター(格闘)", "Mii Brawler", "miifighter"),
            ("Miiファイター(剣術)", "Mii Swordfighter", "miiswordsman"),
            ("Miiファイター(射撃)", "Mii Gunner", "miigunner"),
            ("パルテナ", "Palutena", "palutena"),
            ("パックマン", "PAC-MAN", "pacman"),
            ("ルフレ", "Robin", "reflet"),
            ("シュルク", "Shulk", "shulk"),
            ("クッパJr.", "Bowser Jr.", "koopajr"),
            ("ダックハント", "Duck Hunt", "duckhunt"),
            ("リュウ", "Ryu", "ryu"),
            ("ケン", "Ken", "ken"),
            ("クラウド", "Cloud", "cloud"),
            ("カムイ", "Corrin", "kamui"),
            ("ベヨネッタ", "Bayonetta", "bayonetta"),
            ("インクリング", "Inkling", "inkling"),
            ("リドリー", "Ridley", "ridley"),
            ("シモン", "Simon", "simon"),
            ("リヒター", "Richter", "richter"),
            ("キングクルール", "King K. Rool", "krool"),
            ("しずえ", "Isabelle", "shizue"),
            ("ガオガエン", "Incineroar", "gaogaen"),
            ("パックンフラワー", "Piranha Plant", "packun"),
            ("ジョーカー", "Joker", "jack"),
            ("勇者", "Hero", "brave"),
            ("バンジョー&カズーイ", "Banjo & Kazooie", "buddy"),
            ("テリー", "Terry", "dolly"),
            ("ベレト/ベレス", "Byleth", "master"),
            ("ミェンミェン", "Min Min", "tantan"),
            ("スティーブ/アレックス", "Steve", "pickel"),
            ("セフィロス", "Sephiroth", "edge"),
            ("ホムラ/ヒカリ", "Pyra/Mythra", "eflame"),
            ("カズヤ", "Kazuya", "demon"),
            ("ソラ", "Sora", "trail"),
        ];

        for (name_jp, name_en, fighter_key) in updates {
            db.execute_unprepared(&format!(
                "UPDATE characters SET name_en = '{}', fighter_key = '{}' WHERE name = '{}'",
                name_en, fighter_key, name_jp
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

        // fighter_keyにユニーク制約を追加
        manager
            .create_index(
                Index::create()
                    .name("idx_characters_fighter_key")
                    .table(Characters::Table)
                    .col(Characters::FighterKey)
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
                    .name("idx_characters_fighter_key")
                    .table(Characters::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Characters::Table)
                    .drop_column(Characters::FighterKey)
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
    FighterKey,
}
