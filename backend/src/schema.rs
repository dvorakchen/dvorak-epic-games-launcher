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
        hidden -> Text,
        sort -> Integer,
        game_id -> Integer,
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
    rel_account_game (account_id, game_id) {
        account_id -> Integer,
        game_id -> Integer,
        unlock_achievement_count -> Integer,
    }
}

diesel::table! {
    rel_game_target (game_id, target_id) {
        game_id -> Integer,
        target_id -> Integer,
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

diesel::joinable!(achievements -> games (game_id));
diesel::joinable!(rel_account_game -> accounts (account_id));
diesel::joinable!(rel_account_game -> games (game_id));
diesel::joinable!(rel_game_target -> games (game_id));
diesel::joinable!(rel_game_target -> targets (target_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    achievements,
    games,
    rel_account_game,
    rel_game_target,
    targets,
);
