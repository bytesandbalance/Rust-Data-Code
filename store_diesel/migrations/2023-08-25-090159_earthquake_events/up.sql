-- Your SQL goes here

CREATE TABLE earthquake_events (
    id SERIAL PRIMARY KEY NOT NULL,
    mag FLOAT NOT NULL,
    place TEXT NOT NULL,
    time TIMESTAMP WITH TIME ZONE,
    updated TIMESTAMP WITH TIME ZONE,
    tsunami INT NOT NULL,
    mag_type TEXT NOT NULL,
    event_type TEXT NOT NULL,
    lon FLOAT NOT NULL,
    lat FLOAT NOT NULL
);