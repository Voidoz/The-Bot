-- Your SQL goes here
CREATE TABLE config (
    id INTEGER PRIMARY KEY CHECK (id = 0) NOT NULL,
    token TEXT NOT NULL,
    dev_channel TEXT NOT NULL
)