CREATE TABLE IF NOT EXISTS users (
    id          SERIAL PRIMARY KEY,
    email       VARCHAR NOT NULL CONSTRAINT unique_email_constraint UNIQUE,
    username    VARCHAR NOT NULL CONSTRAINT unique_username_constraint UNIQUE,
    password    VARCHAR NOT NULL,
    id_person   SERIAL REFERENCES users(id)
);
