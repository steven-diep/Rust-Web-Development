-- Add up migration script here
CREATE TABLE IF NOT EXISTS questions (
  id serial PRIMARY KEY,
  title VARCHAR (255) NOT NULL,
  content TEXT NOT NULL,
  tags TEXT [],
  created_on TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO questions (title, content, tags)
VALUES ('Sample Question 1', 'Sample Content 1', ARRAY['Sample', 'Content']);

INSERT INTO questions (title, content, tags)
VALUES ('Sample Question 2', 'Sample Content 2', ARRAY['Example', 'Content']);

INSERT INTO questions (title, content, tags)
VALUES ('Sample Question 3', 'Sample Content 3', ARRAY['FAQ']);


CREATE TABLE IF NOT EXISTS answers (
  id serial PRIMARY KEY,
  content TEXT NOT NULL,
  created_on TIMESTAMP NOT NULL DEFAULT NOW(),
  corresponding_question INTEGER REFERENCES questions(id)
);
