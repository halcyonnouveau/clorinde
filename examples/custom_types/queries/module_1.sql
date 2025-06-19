--: Book() : Default

--! insert_book
INSERT INTO Book (title)
  VALUES (:title);

--! books : Book
SELECT
    Title
FROM
    Book;
