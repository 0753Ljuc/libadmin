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


INSERT INTO books (book_name, book_author, book_publisher, book_price, book_description, category_id1, category_id2, category_id3)
VALUES
('了不起的盖茨比', 'F. Scott 菲茨杰拉德', 'Scribner出版社', 9.99, '一部关于爵士时代衰落的小说。', null,null,null),
('杀死一只知更鸟', 'Harper 李', 'J. B. Lippincott & Co.出版社', 12.50, '一部描绘大萧条时期美国南方的小说。', 1, 2, 5),
('1984', 'George 奥威尔', 'Secker & Warburg出版社', 8.75, '一部描绘极权社会的反乌托邦小说。', 1, 5, null),
('麦田里的守望者', 'J. D. 萨林格', 'Little, Brown and Company出版社', 7.99, '一部描绘一个少年对成人世界的幻灭的小说。', null,null,null),
('霍比特人', 'J. R. R. 托尔金', 'George Allen & Unwin出版社', 10.99, '一部关于一个霍比特人冒险的奇幻小说。', 1, null, null),
('傲慢与偏见', 'Jane 奥斯汀', 'T. Egerton, Whitehall出版社', 11.25, '一部关于班纳特姐妹的爱情纠葛的小说。', null,null,null),
('理智与情感', 'Jane 奥斯汀', 'T. Egerton, Whitehall出版社', 9.99, '一部描绘达什伍德姐妹及其爱情挣扎的小说。', null,null,null),
('爱玛', 'Jane 奥斯汀', 'John Murray出版社', 12.99, '一部描绘主角爱玛的误导性婚姻介绍的小说。', 2, 5, null),
('道林·格雷的画像', 'Oscar 王尔德', 'Ward, Lock, and Company出版社', 6.50, '一部描绘一位男子的肖像仍年轻，而他本人却日渐老化的小说。', null,null,null),
('呼啸山庄', 'Emily 布朗特', 'Thomas Cautley Newby出版社', 7.99, '一部描绘凯瑟琳·恩肖和希斯克利夫命运多舛的爱情故事的小说。', null,null,null);