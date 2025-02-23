-- ユーザー
DROP TABLE IF EXISTS users;
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  photo TEXT,
  bio TEXT DEFAULT 'Hello, I amm a new here!',
  role VARCHAR(50) CHECK (role IN ('user', 'admin', 'creator')) DEFAULT 'user',
  is_verified BOOLEAN DEFAULT FALSE,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- トークン
DROP TABLE IF EXISTS tokens;
CREATE TABLE tokens (
  id SERIAL PRIMARY KEY,
  user_id INTEGER REFERENCES users(id),
  verification_token VARCHAR(255),
  password_reset_token VARCHAR(255),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- タスク
DROP TABLE IF EXISTS tasks;
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  title VARCHAR(100) NOT NULL UNIQUE,
  description TEXT DEFAULT 'No description',
  due_date TIMESTAMP WITH TIME ZONE,
  status VARCHAR(50) CHECK (status IN ('active', 'inactive')) DEFAULT 'active',
  completed BOOL DEFAULT FALSE,
  priority VARCHAR(50) CHECK (priority IN ('low', 'medium', 'high')) DEFAULT 'low',
  user_id INTEGER REFERENCES users(id),
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT fk_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_status ON tasks(status);
CREATE INDEX idx_priority ON tasks(priority);
CREATE INDEX idx_title ON tasks(title);

INSERT INTO users (name, email, password) VALUES
  ('test_user', '123@gmail.com', '$2b$04$BuM27R11fuD0hubq.Nykd.aw.WDI8F2/lYCPabzfLdGG1GHvYqR/i');

INSERT INTO tasks ("title", "description", "due_date", "status", "completed", "priority", "user_id", "created_at", "updated_at") VALUES
	 ('昼寝', '1時間', CURRENT_TIMESTAMP, 'active', false, 'low', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP),
	 ('コーディングテスト', 'Leetcodeでアルゴリズムの勉強', CURRENT_TIMESTAMP, 'active', false, 'low', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);
