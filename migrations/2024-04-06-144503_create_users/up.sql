-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  name TEXT NOT NULL,
  hashed_password TEXT NOT NULL
);
