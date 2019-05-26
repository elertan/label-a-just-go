-- Your SQL goes here
INSERT INTO person (firstname,
                    lastname,
                    created_at)
VALUES ('Dennis',
        'Kievits',
        CURRENT_TIMESTAMP)
        RETURNING id;

INSERT INTO event (name, created_at)
VALUES ('test-event',
        CURRENT_TIMESTAMP);
--
-- INSERT INTO event_invitation (person_id, event_id, token, status)
-- VALUES (
--
--        );