CREATE TABLE IF NOT EXISTS books(
  book_id SERIAL PRIMARY KEY,
  book_name VARCHAR(64) NOT NULL,
  book_author VARCHAR(64) NOT NULL,
  book_publisher VARCHAR(64) NOT NULL,
  book_price DECIMAL(10, 2) NOT NULL,
  book_description VARCHAR(256) NOT NULL,
  category_id1 INTEGER,
  FOREIGN KEY (category_id1) REFERENCES categories(category_id) ON DELETE CASCADE ON UPDATE CASCADE,
  category_id2 INTEGER,
  FOREIGN KEY (category_id2) REFERENCES categories(category_id) ON DELETE CASCADE ON UPDATE CASCADE,
  category_id3 INTEGER,
  FOREIGN KEY (category_id3) REFERENCES categories(category_id) ON DELETE CASCADE ON UPDATE CASCADE,
  status INTEGER NOT NULL DEFAULT 0
);

