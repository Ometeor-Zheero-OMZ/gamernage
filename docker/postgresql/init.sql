-- ユーザー情報
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(60) NOT NULL,
  email VARCHAR(250) NULL,
  profile_image BYTEA NULL,
  password VARCHAR(250) NOT NULL,
  verified_email VARCHAR(250) NULL,
  token TEXT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(name)
);
CREATE UNIQUE INDEX unique_user on users (name);

-- テストユーザーの追加
INSERT INTO users ("name","password") VALUES
	 ('test_user1','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'); -- ゲストログイン対象ユーザー

-- ゲーム情報
DROP TABLE IF EXISTS games;
CREATE TABLE games (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  title VARCHAR(100) NOT NULL,
  description VARCHAR(255) NOT NULL,
  genre VARCHAR(20) NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL
);

CREATE UNIQUE INDEX index_unique_games on games (title);
CREATE INDEX index_games_genre ON games (genre);

-- TODO
DROP TABLE IF EXISTS todos;
CREATE TABLE todos (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id INTEGER NULL,
  game_id INTEGER NULL,
  title VARCHAR(100) NOT NULL,
  description varchar(255) NOT NULL,
  is_completed BOOLEAN NOT NULL,
  status INTEGER NULL,
  priority INTEGER NULL,
  difficulty INTEGER NULL,
  deadline TIMESTAMP NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
  FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE SET NULL
);
CREATE INDEX index_todo_status ON todos (status);
CREATE INDEX index_todo_priority ON todos (priority);
CREATE INDEX index_todo_difficulty ON todos (difficulty);

INSERT INTO todos ("user_id", "title", "description", "is_completed", "status", "priority", "difficulty", "created_at", "updated_at") VALUES
	 (1, 'コーディングテスト', 'Leetcodeでアルゴリズムの勉強', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
	 (1, 'ランニング', '30分くらい公園でランニング', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);