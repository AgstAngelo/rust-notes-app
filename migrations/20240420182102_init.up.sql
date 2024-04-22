-- Add up migration script here
CREATE TABLE users (
  id UUID DEFAULT uuid_generate_v4 (),
  username VARCHAR NOT NULL,
  email VARCHAR UNIQUE,
  password VARCHAR NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE notes (
  id SERIAL,
  user_id UUID REFERENCES users (id) ON DELETE CASCADE,
  title VARCHAR UNIQUE,
  note text NOT NULL,
  tags INTEGER[],
  PRIMARY KEY (id)
);
