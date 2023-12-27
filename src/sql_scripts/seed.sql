DROP TABLE IF EXISTS tutor;
DROP TABLE IF EXISTS topic;

CREATE TABLE tutor (
    id serial primary key,
    first_name varchar(200) not null,
    last_name varchar(200) not null,
    email varchar(200) not null
);

CREATE TABLE topic
(
    id serial primary key,
    tutor_id INT not null,
    title varchar(140) not null,
    topic_description varchar(2000),
    format varchar(30),
    duration varchar(30),
    topic_level varchar(30),
    created_at TIMESTAMP default now(),
    updated_at TIMESTAMP default now(),
    CONSTRAINT fk_tutor
    FOREIGN KEY(tutor_id)
        REFERENCES tutor(id)
        ON DELETE cascade
);

GRANT all privileges ON TABLE tutor TO testuser;
GRANT all privileges ON TABLE topic TO testuser;
GRANT all privileges ON all sequences IN SCHEMA public TO testuser;

SELECT setval('topic_id_seq', 1);
SELECT setval('tutor_id_seq', 1);

/* Load seed data for testing */
INSERT INTO tutor(id, first_name, last_name, email)
VALUES(1,'Mark','Smith','mark@fakemail.com');

INSERT INTO tutor(id, first_name, last_name, email)
VALUES(2,'Frank','Jones','fjones23@fakemail.com');

INSERT INTO tutor(id, first_name, last_name, email)
VALUES(3,'Bob','Lopez','bob.lopez@fakemail.com');


INSERT INTO topic
    (id, tutor_id, title, topic_level, created_at, updated_at)
VALUES(1, 1, 'Traits', 'Beginner' , '2023-03-12 05:40:00', '2023-12-12 05:40:00');

INSERT INTO topic
    (id, tutor_id, title, format, created_at, updated_at)
VALUES(2, 2, 'Lifetimes', 'ebook', '2022-11-12 05:45:00', '2023-11-12 05:45:00');

INSERT INTO topic
    (id, tutor_id, title, format, created_at, updated_at)
VALUES(3, 3, 'Concurrency', 'video', '2021-04-22 05:45:00', '2023-03-12 05:45:00');

INSERT INTO topic
    (id, tutor_id, title, format, created_at, updated_at)
VALUES(4, 3, 'Strings', 'video', '2022-09-22 05:45:00', '2023-10-12 05:45:00');


SELECT setval('tutor_id_seq', (SELECT MAX(id) FROM tutor) + 1);
SELECT setval('topic_id_seq', (SELECT MAX(id) FROM topic) + 1);