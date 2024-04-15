CREATE TABLE users (
  id UUID PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  name TEXT NOT NULL,
  hashed_password TEXT NOT NULL
);
