-- Your SQL goes here
CREATE TABLE posts_tags (
    post_id INTEGER NOT NULL REFERENCES posts(id),
    tag VARCHAR NOT NULL,
    PRIMARY KEY (post_id, tag)
);