PRAGMA foreign_keys=ON;

CREATE TABLE nodes 
(
    id      INTEGER PRIMARY KEY,
    name    TEXT,
    url     TEXT,
    comment TEXT
);

CREATE TABLE edges
(
    source INTEGER NOT NULL,
    dest   INTEGER NOT NULL,
    FOREIGN KEY (source) REFERENCES nodes(id),
    FOREIGN KEY (dest)   REFERENCES nodes(id),
    UNIQUE (source, dest)
);

CREATE TABLE entries
(
    parent  INTEGER NOT NULL DEFAULT 0,
    date    TEXT DEFAULT CURRENT_DATE,
    comment TEXT,
    code    TEXT,
    FOREIGN KEY (parent) REFERENCES nodes(id)
);