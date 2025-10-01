-- スマブラSP 戦績管理データベース スキーマ
-- PostgreSQL 用

-- キャラクターマスタ
CREATE TABLE characters (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL UNIQUE,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- セッション（1日の対戦記録）
CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  session_date DATE NOT NULL,
  notes TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- マッチ（個別の対戦）
CREATE TABLE matches (
  id SERIAL PRIMARY KEY,
  session_id INTEGER NOT NULL REFERENCES sessions(id) ON DELETE CASCADE,
  character_id INTEGER NOT NULL REFERENCES characters(id),
  opponent_character_id INTEGER NOT NULL REFERENCES characters(id),
  result VARCHAR(10) NOT NULL CHECK (result IN ('win', 'loss')),
  match_order INTEGER NOT NULL,
  gsp_before INTEGER,
  gsp_after INTEGER,
  comment TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(session_id, match_order)
);

-- インデックス
CREATE INDEX idx_sessions_date ON sessions(session_date);
CREATE INDEX idx_matches_session ON matches(session_id);
CREATE INDEX idx_matches_character ON matches(character_id);
CREATE INDEX idx_matches_opponent ON matches(opponent_character_id);
CREATE INDEX idx_matches_created_at ON matches(created_at);

-- updated_at自動更新
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_characters_updated_at
  BEFORE UPDATE ON characters
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sessions_updated_at
  BEFORE UPDATE ON sessions
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_matches_updated_at
  BEFORE UPDATE ON matches
  FOR EACH ROW
  EXECUTE FUNCTION update_updated_at_column();
