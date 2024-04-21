-- Add up migration script here
CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4 (),
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE notes (
  id serial,
  title VARCHAR NOT NULL,
  note text NOT NULL,
  tags integer[],
  PRIMARY KEY (id)
);
