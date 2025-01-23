// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        rank -> Text,
        registered_at -> Timestamp,
    }
}
