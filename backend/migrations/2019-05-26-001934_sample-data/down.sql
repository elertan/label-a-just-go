-- This file should undo anything in `up.sql`
DELETE FROM person WHERE firstname = 'Dennis' AND lastname = 'Kievits';
DELETE FROM event WHERE name = 'test-event';