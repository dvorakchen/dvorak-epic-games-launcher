-- Your SQL goes here
CREATE TABLE
    games (
        id integer primary key not null unique,
        name text not null default '',
        -- sparated by ';', e.g: nice;battle
        review_label text not null default '',
        summary text not null default '',
        score integer not null default 0,
        detail text not null default '',
        rating_system text not null default '',
        rating text not null default '',
        developer text not null default '',
        publisher text not null default '',
        release_date text not null default '',
        -- sparated by ';', e.g WINDOWS;MAC
        platform text not null default '',
        create_date_time text not null default (datetime('now', 'localtime', 'utc'))
    );

CREATE TABLE
    targets (
        id integer primary key not null unique,
        -- genres, features
        type text not null default '',
        name text not null default '',
        link text not null default '',
        sort integer not null default 0
    );

CREATE TABLE
    rel_game_target (
        id integer primary key not null unique,
        games_key integer REFERENCES games (id),
        target_key integer REFERENCES targets (id)
    );

CREATE TABLE
    achievements (
        id integer primary key not null unique,
        cover_link text not null default '',
        cover_alt text not null default '',
        title text not null default '',
        summary text not null default '',
        ex integer not null default 0,
        unlocks integer not null default 0,
        -- is hidden, TRUE or FALSE
        hidden text not null default 'FALSE',
        sort integer not null default 0
    );

CREATE TABLE
    rel_game_achievement (
        id integer primary key not null unique,
        games_key integer REFERENCES games (id),
        achievement_key integer REFERENCES achievements (id)
    );

CREATE TABLE
    rel_account_game (
        id integer primary key not null unique,
        accounts_key integer REFERENCES accounts (id),
        games_key integer REFERENCES games (id)
    );