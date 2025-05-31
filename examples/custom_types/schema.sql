CREATE TABLE Author (
    Id serial NOT NULL,
    Name varchar(70) NOT NULL,
    Country varchar(100) NOT NULL,
    Dob date NOT NULL,
    PRIMARY KEY (Id)
);

INSERT INTO Author (Name, Country, Dob)
    VALUES ('Agatha Christie', 'United Kingdom', '1999-01-02'), ('John Ronald Reuel Tolkien', 'United Kingdom', '2003-02-1');

CREATE TABLE Book (
    Id serial NOT NULL,
    Title varchar(50) NOT NULL,
    Translations text[] NOT NULL DEFAULT ARRAY['french', 'english'],
    PRIMARY KEY (Id)
);

INSERT INTO Book (Title)
    VALUES ('Murder on the Orient Express'), ('Death on the Nile'), ('The Hobbit'), ('The Silmarillion');

CREATE TABLE BookAuthor (
    AuthorId int NOT NULL,
    BookId int NOT NULL,
    FOREIGN KEY (AuthorId) REFERENCES Author (Id),
    FOREIGN KEY (BookId) REFERENCES Book (Id)
);

INSERT INTO BookAuthor (AuthorId, BookId)
    VALUES (1, 1), (1, 2), (2, 3), (2, 4);

CREATE TYPE Element AS ENUM (
    'Anemo',
    'Cryo',
    'Dendro',
    'Electro',
    'Geo',
    'Hydro',
    'Pyro',
    'Physical'
);

CREATE TYPE Sponge_Bob_Character AS enum (
    'Bob',
    'Patrick',
    'Squidward'
);

CREATE TYPE VoiceActor AS (
    name text,
    age integer
);

CREATE TABLE SpongeBobVoiceActor (
    voice_actor VoiceActor,
    element Element,
    character Sponge_Bob_Character
);

INSERT INTO SpongeBobVoiceActor (voice_actor, element, character)
    VALUES (ROW ('Bill Fagerbakke', 65), 'Hydro', 'Patrick');
