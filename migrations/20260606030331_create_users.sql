CREATE TABLE users (
    email TEXT PRIMARY KEY NOT NULL UNIQUE,
    is_admin INTEGER NOT NULL CHECK (is_admin IN (0,1)) DEFAULT 0,
    emails_enabled INTEGER NOT NULL CHECK (emails_enabled IN (0,1)) DEFAULT 1,
    send_time TEXT NOT NULL DEFAULT '00:00'
);
