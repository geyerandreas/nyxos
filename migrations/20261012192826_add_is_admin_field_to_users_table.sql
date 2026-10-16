-- Add migration script here
ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT 0 CHECK (is_admin IN (0, 1));

CREATE TABLE ApiTokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL UNIQUE, -- SHA-256 Hash des Tokens
    name TEXT NOT NULL,              -- Name des Tokens (z. B. "Laptop-Twine")
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
