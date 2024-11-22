-- Add migration script here
CREATE TABLE todo (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    todo TEXT NOT NULL,
    is_done BOOL
);
