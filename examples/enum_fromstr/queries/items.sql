-- Query to select all items
--! get_all_items
SELECT id, name, color, status 
FROM items;

-- Query to filter items by color
--! get_items_by_color
SELECT id, name, color, status 
FROM items
WHERE color = :color;

-- Query to filter items by status
--! get_items_by_status
SELECT id, name, color, status 
FROM items
WHERE status = :status;

-- Query to create a new item
--! create_item
INSERT INTO items (name, color, status)
VALUES (:name, :color, :status)
RETURNING id, name, color, status;