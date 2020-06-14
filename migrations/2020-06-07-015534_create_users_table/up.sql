-- Your SQL goes here
CREATE TABLE users (
  id VARCHAR(36) PRIMARY KEY UNIQUE NOT NULL,
  user_name VARCHAR(64) UNIQUE NOT NULL,
  email VARCHAR(320) UNIQUE NOT NULL,
  password TEXT NOT NULL,
  first_name TEXT NULL,
  last_name TEXT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMP NULL
);