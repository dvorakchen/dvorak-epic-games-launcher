// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Integer,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}
