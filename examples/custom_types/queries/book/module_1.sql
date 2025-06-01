--! insert_book
INSERT INTO Book (title)
  VALUES (:title);

--! books
SELECT
    Title
FROM
    Book;
