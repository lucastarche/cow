PRAGMA foreign_keys=ON;

CREATE TABLE folders
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    parent       INTEGER,
    description  TEXT,
    FOREIGN KEY (parent) REFERENCES folders(parent)
);

CREATE TABLE problems 
(
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    name         TEXT NOT NULL,
    description  TEXT
);

CREATE TABLE entries
(
    parent      INTEGER NOT NULL,
    date        TEXT DEFAULT CURRENT_DATE,
    description TEXT,
    code        TEXT,
    FOREIGN KEY (parent) REFERENCES problems(id)
);

CREATE TABLE problem_folders
(
    folder_id   INTEGER NOT NULL,
    problem_id  INTEGER NOT NULL,
    FOREIGN KEY (folder_id) REFERENCES folders(id),
    FOREIGN KEY (problem_id) REFERENCES problems(id)
);