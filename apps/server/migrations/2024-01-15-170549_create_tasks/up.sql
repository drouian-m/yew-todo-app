-- Your SQL goes here
CREATE TABLE tasks (
  id VARCHAR(21) NOT NULL PRIMARY KEY,
  title VARCHAR NOT NULL,
  completed BOOLEAN NOT NULL,
  created_at TIMESTAMP NOT NULL
)
