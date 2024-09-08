-- ユーザー
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT,
  email TEXT UNIQUE,
  email_verified TIMESTAMP WITH TIME ZONE,
  username TEXT UNIQUE,
  image TEXT,
  profile_image BYTEA NULL,
  password VARCHAR(250) NOT NULL,
  token TEXT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- アカウント
DROP TABLE IF EXISTS account;
CREATE TABLE account (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id TEXT NOT NULL,
  type TEXT NOT NULL,
  provider TEXT NOT NULL,
  provider_account_id TEXT NOT NULL,
  refresh_token TEXT,
  access_token TEXT,
  expires_at INT,
  token_type TEXT,
  scope TEXT,
  id_token TEXT,
  session_state TEXT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT unique_provider_provider_account_id UNIQUE (provider, provider_account_id)
);

-- セッション
DROP TABLE IF EXISTS session;
CREATE TABLE session (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  session_token TEXT UNIQUE NOT NULL,
  user_id TEXT NOT NULL,
  expires TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user_session FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- スレッド
DROP TABLE IF EXISTS subreddit;
CREATE TABLE subreddit (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT UNIQUE NOT NULL,
  creator_id TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_creator FOREIGN KEY (creator_id) REFERENCES users(id)
);

-- 定期購読
DROP TABLE IF EXISTS subscription;
CREATE TABLE subscription (
  user_id TEXT NOT NULL,
  subreddit_id TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT pk_subscription PRIMARY KEY (user_id, subreddit_id),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id),
  CONSTRAINT fk_subreddit FOREIGN KEY (subreddit_id) REFERENCES subreddit(id)
);

-- 投稿
DROP TABLE IF EXISTS post;
CREATE TABLE post (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  content JSONB,
  author_id TEXT NOT NULL,
  subreddit_id TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES users(id),
  CONSTRAINT fk_subreddit FOREIGN KEY (subreddit_id) REFERENCES subreddit(id)
);

-- コメント
DROP TABLE IF EXISTS comment;
CREATE TABLE comment (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  text TEXT NOT NULL,
  author_id TEXT NOT NULL,
  post_id TEXT NOT NULL,
  reply_to_id TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_author FOREIGN KEY (author_id) REFERENCES users(id),
  CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
  CONSTRAINT fk_reply_to FOREIGN KEY (reply_to_id) REFERENCES comment(id) ON DELETE RESTRICT
);

-- 投票
DROP TABLE IF EXISTS vote;
CREATE TABLE vote (
  user_id TEXT NOT NULL,
  post_id TEXT NOT NULL,
  type TEXT CHECK (type IN ('UP', 'DOWN')) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT pk_vote PRIMARY KEY (user_id, post_id),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id),
  CONSTRAINT fk_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
);

-- コメント投票
DROP TABLE IF EXISTS comment_vote;
CREATE TABLE comment_vote (
  user_id TEXT NOT NULL,
  comment_id TEXT NOT NULL,
  type TEXT CHECK (type IN ('UP', 'DOWN')) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT pk_comment_vote PRIMARY KEY (user_id, comment_id),
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id),
  CONSTRAINT fk_comment FOREIGN KEY (comment_id) REFERENCES comment(id) ON DELETE CASCADE
);

-- ゲーム
DROP TABLE IF EXISTS games;
CREATE TABLE games (
  id TEXT PRIMARY KEY DEFAULT gen_random_uuid(),
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  genre TEXT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP,
  CONSTRAINT unique_title UNIQUE (title)
);

-- TODO
DROP TABLE IF EXISTS todos;
CREATE TABLE todos (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id TEXT,
  game_id TEXT,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  is_completed BOOLEAN NOT NULL,
  status INTEGER,
  priority INTEGER,
  difficulty INTEGER,
  deadline TIMESTAMP,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
  FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE SET NULL
);

CREATE INDEX index_todo_status ON todos (status);
CREATE INDEX index_todo_priority ON todos (priority);
CREATE INDEX index_todo_difficulty ON todos (difficulty);

---- ↓↓↓データ挿入↓↓↓
-- テストユーザーの追加
INSERT INTO users (name, password) VALUES
    ('test_user1', '$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'); -- ゲストログイン対象ユーザー

INSERT INTO todos ("user_id", "title", "description", "is_completed", "status", "priority", "difficulty", "created_at", "updated_at") VALUES
	 ((SELECT id FROM users WHERE name = 'test_user1'), 'コーディングテスト', 'Leetcodeでアルゴリズムの勉強', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
	 ((SELECT id FROM users WHERE name = 'test_user1'), 'ランニング', '30分くらい公園でランニング', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);