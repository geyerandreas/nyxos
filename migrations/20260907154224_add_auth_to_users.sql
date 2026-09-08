-- Add password and auth fields to users table
ALTER TABLE users ADD COLUMN password_hash TEXT NOT NUll DEFAULT '';
ALTER TABLE users ADD COLUMN created_at DATETIME DEFAULT CURRENT_TIMESTAMP;

-- Token revocation list (for logout functionality)
CREATE TABLE IF NOT EXISTS RevokedTokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    token_jti TEXT NOT NUll UNIQUE,
    user_id INTEGER NOT NULL,
    revoked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE INDEX idx_revoked_tokens_user_id ON RevokedTokens(user_id);
