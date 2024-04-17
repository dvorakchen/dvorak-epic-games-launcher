-- Your SQL goes here
CREATE TABLE
    accounts (
        id integer primary key not null unique,
        email text not null,
        password text not null
    )