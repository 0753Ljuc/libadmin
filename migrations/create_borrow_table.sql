CREATE TABLE IF NOT EXISTS borrows (
  borrow_id SERIAL PRIMARY KEY,
  book_id INTEGER NOT NULL,
  FOREIGN KEY (book_id) REFERENCES books(book_id),
  user_id INTEGER NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  borrow_date DATE NOT NULL DEFAULT CURRENT_DATE
);