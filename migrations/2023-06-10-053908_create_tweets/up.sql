-- Your SQL goes here
CREATE TABLE tweets (
  id SERIAL NOT NULL PRIMARY KEY,
  message VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL
)