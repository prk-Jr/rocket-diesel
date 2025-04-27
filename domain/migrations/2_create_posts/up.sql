CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    created_by INTEGER REFERENCES users(id),
    title VARCHAR NOT NULL,
    body TEXT NOT NULL
);