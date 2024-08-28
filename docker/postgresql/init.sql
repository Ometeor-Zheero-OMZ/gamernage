-- Create restaurant tables table
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS restaurant_tables;
CREATE TABLE restaurant_tables (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  table_number INTEGER not null,
  note VARCHAR(500) DEFAULT null,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX index_table on restaurant_tables (table_number);

-- Adds 10 tables
INSERT INTO restaurant_tables (table_number, note) VALUES (1, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (2, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (3, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (4, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (5, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (6, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (7, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (8, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (9, null);
INSERT INTO restaurant_tables (table_number, note) VALUES (10, null);

-- ユーザー情報
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(60) NOT NULL,
  email VARCHAR(250) NULL,
  profile_image BYTEA NULL,
  password VARCHAR(61) NOT NULL,
  verified_email VARCHAR(250) NULL,
  token TEXT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE(name)
);
CREATE UNIQUE INDEX unique_user on users (name);

-- テストユーザーの追加
INSERT INTO users ("name","password") VALUES
	 ('test_user1','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user2','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user3','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user4','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user5','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user6','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user7','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user8','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user9','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user10','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
   ('test_user11','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user12','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user13','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user14','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user15','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user16','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user17','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user18','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user19','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i'),
	 ('test_user20','$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i');

-- Create menus table
DROP TABLE IF EXISTS menus;
CREATE TABLE menus (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  name VARCHAR(50) NOT NULL,
  cook_time_seconds INTEGER NOT null,
  price integer NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX index_unique_menu on menus (name);
CREATE INDEX index_menu on menus (name, cook_time_seconds);

-- add menus
INSERT INTO menus ("name","cook_time_seconds","price") VALUES
	 ('Hamburger',300, 500), -- 5 minutes
   ('Curry rice',300, 500), -- 5 minutes
	 ('Special Hamburger',600, 1000), -- 10 minutes
   ('Special soup',900, 1000), -- 15 minutes
   ('Sushi',900, 500), -- 15 minutes
   ('Potato',300, 500), -- 5 minute
   ('Cheese Hamburger',300, 1000), -- 5 minutes
   ('Rice',300, 500), -- 5 minutes
	 ('Churros',600, 500), -- 10 minutes
   ('Ice cream',300, 500), -- 5 minutes
   ('Roll cake',900, 500), -- 15 minutes
   ('Small pancake',300, 500), -- 5 minute
   ('Bread',300, 500), -- 5 minutes
   ('Cola',300, 500), -- 5 minute
   ('Strawberry cake',600, 500), -- 10 minutes
   ('Tea',300, 500), -- 5 minute
   ('Cheese Cake',900, 1000), -- 15 minutes
   ('Potato soup',300, 500), -- 5 minute
   ('Hamburger and cola set',900, 500), -- 15 minutes
   ('Curry rice and cola set',900, 500), -- 15 minutes
	 ('Special Hamburger and cola set',900, 1500), -- 15 minutes
   ('Sushi and cola set',900, 1500), -- 15 minutes
   ('Pancake set',900, 1500), -- 15 minutes;
   ('Wine',900, 500), -- 15 minutes
   ('Onion soup',300, 500), -- 5 minute
   ('Carrot soup',900, 500), -- 15 minutes
   ('Shrimps',900, 500), -- 15 minutes
	 ('Ham',900, 500), -- 15 minutes
   ('Beer',900, 500), -- 15 minutes
   ('Coffee',900, 500); -- 15 minutes;

-- Create orders table
CREATE TABLE orders (
  id BIGSERIAL PRIMARY KEY,
  restaurant_table_id INTEGER NOT null REFERENCES restaurant_tables (id),
  menu_id INTEGER NOT null REFERENCES menus (id),
  expected_cook_finish_time TIMESTAMP NOT null,
  is_served_by_staff BOOLEAN NOT NULL,
  served_by_user_id INTEGER DEFAULT null REFERENCES users (id),
  checked_by_user_id INTEGER DEFAULT null REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TIMESTAMP DEFAULT NULL
);
CREATE INDEX index_orders on orders (restaurant_table_id, menu_id, served_by_user_id, checked_by_user_id);

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