-- Query to get all navigation points
--! get_all_navigation
SELECT id, point_name, heading
FROM navigation;

-- Query to get navigation points by direction
--! get_navigation_by_direction
SELECT id, point_name
FROM navigation
WHERE heading = :direction;