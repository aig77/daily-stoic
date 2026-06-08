CREATE TABLE login_codes (
  email TEXT PRIMARY KEY NOT NULL UNIQUE, 
  code TEXT NOT NULL,
  expires_at TEXT NOT NULL
);
