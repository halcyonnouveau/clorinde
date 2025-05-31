--: Author() : Default

--! authors : Author
SELECT
    *
FROM
    Author;

--! author_name_by_id
SELECT
    Author.Name
FROM
    Author
WHERE
    Author.Id = :id;

--! author_name_starting_with AuthorNameStartingWithParams()
SELECT
    BookAuthor.AuthorId,
    Author.Name,
    BookAuthor.BookId,
    Book.Title
FROM
    BookAuthor
    INNER JOIN Author ON Author.id = BookAuthor.AuthorId
    INNER JOIN Book ON Book.Id = BookAuthor.BookId
WHERE
    Author.Name LIKE CONCAT(:start_str::text, '%');

--! select_voice_actor_by_element
SELECT
    voice_actor, element, character
FROM
    SpongeBobVoiceActor
WHERE
    element = :element;
