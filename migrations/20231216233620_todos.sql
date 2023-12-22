CREATE TABLE IF NOT EXISTS todos
(
    id INTEGER PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS users
(
    id INTEGER PRIMARY KEY NOT NULL,
    username TEXT NOT NULL unique,
    password TEXT NOT NULL
);

INSERT INTO users(id, username, password)
values (1, 'ferris', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');
