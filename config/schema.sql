CREATE TABLE IF NOT EXISTS highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT,
    page TEXT,
    location TEXT NOT NULL,
    location_start INTEGER NOT NULL,
    location_end INTEGER,
    date TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT NOT NULL,
    highlight_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (highlight_id) REFERENCES highlights(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_title ON highlights(title);
CREATE INDEX IF NOT EXISTS idx_date ON highlights(date);
CREATE INDEX IF NOT EXISTS idx_location_start ON highlights(location_start);
CREATE INDEX IF NOT EXISTS idx_location_end ON highlights(location_end);
CREATE INDEX IF NOT EXISTS idx_notes_highlight_id ON notes(highlight_id);
