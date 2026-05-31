CREATE TABLE quotes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL UNIQUE,
    month_topic TEXT,
    season_topic TEXT,
    title TEXT,
    quote TEXT,
    quoter TEXT,
    explanation TEXT
);
