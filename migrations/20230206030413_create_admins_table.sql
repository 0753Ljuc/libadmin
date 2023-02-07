

CREATE TABLE IF NOT EXISTS admins   (
  id SERIAL PRIMARY KEY,
  name VarChar(50) NOT NULL UNIQUE,
  gender CHAR(1) NOT NULL,
  phone_number VARCHAR(18) NOT NULL,
  id_card INTEGER,
  borrow_card INTEGER,
  permission BOOLEAN NOT NULL,
  hash VarChar(256) NOT NULL
);

