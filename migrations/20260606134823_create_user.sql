CREATE TABLE users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    master_id UUID REFERENCES users(id)
);

