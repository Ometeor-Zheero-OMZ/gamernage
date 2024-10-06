-- ユーザー
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ユーザープロファイル
DROP TABLE IF EXISTS user_profiles;
CREATE TABLE user_profiles (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  user_id INT UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  email TEXT UNIQUE,
  email_verified TIMESTAMP WITH TIME ZONE,
  username TEXT UNIQUE NOT NULL,
  image TEXT,
  header_image TEXT,
  profile_image BYTEA NULL,
  bio TEXT,
  onboarded BOOLEAN DEFAULT FALSE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_profiles_email ON user_profiles(email);
CREATE INDEX idx_user_profiles_username ON user_profiles(username);

-- 認証ユーザー
DROP TABLE IF EXISTS user_auth;
CREATE TABLE user_auth (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  user_id INT UNIQUE REFERENCES users(id) ON DELETE CASCADE,
  password VARCHAR(512) NOT NULL,
  token TEXT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_auth_user_id ON user_auth(user_id);

-- アカウント
DROP TABLE IF EXISTS account;
CREATE TABLE account (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  user_id INT NOT NULL,
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
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  session_token TEXT UNIQUE NOT NULL,
  user_id INT NOT NULL,
  expires TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user_session FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- スレッド
DROP TABLE IF EXISTS threads;
CREATE TABLE threads (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  text TEXT NOT NULL,
  author INT NOT NULL,
  community INT,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  parent_id INT,
  FOREIGN KEY (author) REFERENCES users(id) ON DELETE CASCADE,
  FOREIGN KEY (community) REFERENCES communities(id) ON DELETE SET NULL,
  FOREIGN KEY (parent_id) REFERENCES threads(id) ON DELETE SET NULL
);

-- 子スレッド
DROP TABLE IF EXISTS thread_children;
CREATE TABLE thread_children (
  thread_id INT,
  child_id INT,
  PRIMARY KEY (thread_id, child_id),
  FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE,
  FOREIGN KEY (child_id) REFERENCES threads(id) ON DELETE CASCADE
);

-- コミュニティ
DROP TABLE IF EXISTS communities;
CREATE TABLE communities (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  username VARCHAR UNIQUE NOT NULL,
  name VARCHAR NOT NULL,
  image TEXT,
  bio TEXT,
  created_by INT REFERENCES users(id) ON DELETE SET NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- コミュニティメンバー
DROP TABLE IF EXISTS community_members;
CREATE TABLE community_members (
  community_id INT,
  user_id INT,
  PRIMARY KEY (community_id, user_id),
  FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- コミュニティに関連するスレッド
DROP TABLE IF EXISTS community_threads;
CREATE TABLE community_threads (
  community_id INT,
  thread_id INT,
  PRIMARY KEY (community_id, thread_id),
  FOREIGN KEY (community_id) REFERENCES communities(id) ON DELETE CASCADE,
  FOREIGN KEY (thread_id) REFERENCES threads(id) ON DELETE CASCADE
);

-- ゲーム
DROP TABLE IF EXISTS games;
CREATE TABLE games (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  genre VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP
);

-- トレーニングメニュー
DROP TABLE IF EXISTS training_menus;
CREATE TABLE training_menus (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  user_id INT,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL
);

-- 複数ゲームタイトルとトレーニングメニューを関連付けるための中間テーブル
DROP TABLE IF EXISTS training_menu_game;
CREATE TABLE training_menu_game (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  training_menu_id INT,
  game_id INT,
  FOREIGN KEY (training_menu_id) REFERENCES training_menus(id) ON DELETE SET NULL,
  FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE SET NULL
);

-- トレーニングメニューアイテム
DROP TABLE IF EXISTS menu_items;
CREATE TABLE menu_items (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  training_menu_id INT,
  step_number INT,
  action VARCHAR,
  target VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (training_menu_id) REFERENCES training_menus(id) ON DELETE SET NULL
);

-- ユーザーが追加したトレーニングメニュー
DROP TABLE IF EXISTS user_training_menus;
CREATE TABLE user_training_menus (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  user_id INT,
  training_menu_id INT,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE SET NULL,
  FOREIGN KEY (training_menu_id) REFERENCES training_menus(id) ON DELETE SET NULL
);

-- メニュータグ
DROP TABLE IF EXISTS menu_tags;
CREATE TABLE menu_tags (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 複数トレーニングメニューとタグを関連付けるための中間テーブル
DROP TABLE IF EXISTS menu_tag;
CREATE TABLE menu_tag (
  public_id UUID UNIQUE DEFAULT gen_random_uuid(),
  training_menu_id INT,
  tag_id INT,
  FOREIGN KEY (training_menu_id) REFERENCES training_menus(id) ON DELETE SET NULL,
  FOREIGN KEY (tag_id) REFERENCES menu_tags(id) ON DELETE SET NULL
);

-- TODO
DROP TABLE IF EXISTS todos;
CREATE TABLE todos (
  id INT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  user_id INT,
  game_id INT,
  title VARCHAR NOT NULL,
  description TEXT NOT NULL,
  is_completed BOOLEAN NOT NULL,
  status INT,
  priority INT,
  difficulty INT,
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
INSERT INTO users (created_at, updated_at)
VALUES (CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- ユーザープロファイルの追加
INSERT INTO user_profiles (user_id, name, email, created_at, updated_at)
VALUES (1, 'test_user1', 'test@gmail.com', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- 認証情報の追加
INSERT INTO user_auth (user_id, password, created_at, updated_at)
VALUES (1, '$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);


INSERT INTO todos ("user_id", "title", "description", "is_completed", "status", "priority", "difficulty", "created_at", "updated_at") VALUES
	 ((SELECT user_id FROM user_profiles WHERE name = 'test_user1'), 'コーディングテスト', 'Leetcodeでアルゴリズムの勉強', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
	 ((SELECT user_id FROM user_profiles WHERE name = 'test_user1'), 'ランニング', '30分くらい公園でランニング', false, 1, 1, 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);