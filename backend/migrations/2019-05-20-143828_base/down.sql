-- This file should undo anything in `up.sql`
DROP TABLE photo;
DROP TABLE event_invitation;
DROP TABLE event;
DROP TABLE person;
DROP EXTENSION IF EXISTS "uuid-ossp";