-- Example schema for enum FromStr example

CREATE TYPE color AS ENUM ('red', 'green', 'blue');
CREATE TYPE status AS ENUM ('pending', 'active', 'inactive');
CREATE TYPE direction AS ENUM ('north', 'south', 'east', 'west');

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    color color NOT NULL,
    status status NOT NULL
);

INSERT INTO items (name, color, status) VALUES
    ('Item 1', 'red', 'active'),
    ('Item 2', 'blue', 'pending'),
    ('Item 3', 'green', 'inactive'),
    ('Item 4', 'red', 'inactive');

CREATE TABLE navigation (
    id SERIAL PRIMARY KEY,
    point_name TEXT NOT NULL,
    heading direction NOT NULL
);

INSERT INTO navigation (point_name, heading) VALUES
    ('Waypoint 1', 'north'),
    ('Waypoint 2', 'east'),
    ('Waypoint 3', 'south'),
    ('Waypoint 4', 'west');