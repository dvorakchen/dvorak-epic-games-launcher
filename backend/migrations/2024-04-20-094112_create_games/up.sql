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
        create_date_time text not null default (datetime ('now', 'localtime', 'utc'))
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
        game_id integer not null REFERENCES games (id),
        target_id integer not null REFERENCES targets (id),
        PRIMARY KEY (game_id, target_id)
    );

CREATE TABLE
    achievements (
        id integer primary key not null unique,
        cover_link text not null default '',
        cover_alt text not null default '',
        title text not null default '',
        summary text not null default '',
        ex integer not null default 0,
        -- is hidden, TRUE or FALSE
        hidden text not null default 'FALSE',
        sort integer not null default 0,
        game_id integer not null REFERENCES games (id)
    );

CREATE TABLE
    rel_account_game (
        account_id integer not null REFERENCES accounts (id),
        game_id integer not null REFERENCES games (id),
        unlock_achievement_count integer not null default 0,
        PRIMARY KEY (account_id, game_id)
    );