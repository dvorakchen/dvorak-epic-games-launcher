// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}

diesel::table! {
    achievements (id) {
        id -> Integer,
        cover_link -> Text,
        cover_alt -> Text,
        title -> Text,
        summary -> Text,
        ex -> Integer,
        unlocks -> Integer,
        hidden -> Text,
        sort -> Integer,
    }
}

diesel::table! {
    games (id) {
        id -> Integer,
        name -> Text,
        review_label -> Text,
        summary -> Text,
        score -> Integer,
        detail -> Text,
        rating_system -> Text,
        rating -> Text,
        developer -> Text,
        publisher -> Text,
        release_date -> Text,
        platform -> Text,
        create_date_time -> Text,
    }
}

diesel::table! {
    rel_account_game (id) {
        id -> Integer,
        accounts_key -> Nullable<Integer>,
        games_key -> Nullable<Integer>,
    }
}

diesel::table! {
    rel_game_achievement (id) {
        id -> Integer,
        games_key -> Nullable<Integer>,
        achievement_key -> Nullable<Integer>,
    }
}

diesel::table! {
    rel_game_target (id) {
        id -> Integer,
        games_key -> Nullable<Integer>,
        target_key -> Nullable<Integer>,
    }
}

diesel::table! {
    targets (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        name -> Text,
        link -> Text,
        sort -> Integer,
    }
}

diesel::joinable!(rel_account_game -> accounts (accounts_key));
diesel::joinable!(rel_account_game -> games (games_key));
diesel::joinable!(rel_game_achievement -> achievements (achievement_key));
diesel::joinable!(rel_game_achievement -> games (games_key));
diesel::joinable!(rel_game_target -> games (games_key));
diesel::joinable!(rel_game_target -> targets (target_key));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    achievements,
    games,
    rel_account_game,
    rel_game_achievement,
    rel_game_target,
    targets,
);
