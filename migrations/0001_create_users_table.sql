CREATE TABLE IF NOT EXISTS logs (
    id SERIAL PRIMARY KEY,
    label TEXT NOT NULL,
    timestamp bigint NOT NULL
);

