CREATE TABLE quotes (
    date TEXT PRIMARY KEY NOT NULL UNIQUE,
    month_topic TEXT NOT NULL,
    season_topic TEXT NOT NULL,
    title TEXT NOT NULL,
    quote TEXT NOT NULL,
    quoter TEXT NOT NULL,
    explanation TEXT NOT NULL
);
