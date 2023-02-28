CREATE TABLE IF NOT EXISTS categories(
  category_id SERIAL PRIMARY KEY,
  category_name VarChar(64) NOT NULL UNIQUE
);

INSERT INTO
  categories (category_name)
VALUES
  ('Mathematics'),
  ('Science'),
  ('History'),
  ('Literature'),
  ('Language'),
  ('Computer Science'),
  ('Art'),
  ('Music'),
  ('Physical Education'),
  ('Business');